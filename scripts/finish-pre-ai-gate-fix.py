from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

# MemoryOrigin::as_str is only part of the storage round-trip unit test. Production only needs
# from_str when decoding durable SQLite provenance, so keep the helper out of the lib target.
memory_path = ROOT / "src-tauri/src/domain/memory.rs"
memory = memory_path.read_text(encoding="utf-8")
old = "impl MemoryOrigin {\n    pub fn as_str(self) -> &'static str {"
new = "impl MemoryOrigin {\n    #[cfg(test)]\n    pub fn as_str(self) -> &'static str {"
if memory.count(old) != 1:
    raise RuntimeError(f"MemoryOrigin as_str shape changed: {memory.count(old)} matches")
memory_path.write_text(memory.replace(old, new), encoding="utf-8", newline="\n")

# The target machine is Windows 11 and may invoke the repo script through Windows PowerShell 5.1.
# Keep the harness deliberately conservative: no null-coalescing operator and no statement-valued
# hashtable entries. CPU/RAM collection uses only built-in CIM/Get-Process, GPU counters remain
# best-effort. No global install is needed.
script = r'''param(
  [string]$ExecutablePath = "src-tauri/target/release/northpalace-my-pet.exe",
  [string]$Scenario = "idle",
  [int]$WarmupSeconds = 10,
  [int]$SampleSeconds = 60,
  [int]$IntervalMs = 1000,
  [switch]$Launch,
  [string]$OutputDirectory = ".workspace/benchmarks"
)

Set-StrictMode -Version 2.0
$ErrorActionPreference = "Stop"

function Get-DescendantIds([int]$RootId) {
  $rows = @(Get-CimInstance Win32_Process | Select-Object ProcessId, ParentProcessId)
  $ids = @($RootId)
  $changed = $true
  while ($changed) {
    $changed = $false
    foreach ($row in $rows) {
      $parentId = [int]$row.ParentProcessId
      $childId = [int]$row.ProcessId
      if (($ids -contains $parentId) -and ($ids -notcontains $childId)) {
        $ids += $childId
        $changed = $true
      }
    }
  }
  return @($ids)
}

function Get-GpuSample([int[]]$Ids) {
  $result = @{ total = $null; maxEngine = $null }
  try {
    $counterSet = Get-Counter '\GPU Engine(*)\Utilization Percentage' -ErrorAction Stop
    $values = @()
    foreach ($counter in $counterSet.CounterSamples) {
      if ($counter.Path -match 'pid_(\d+)_') {
        $counterPid = [int]$Matches[1]
        if ($Ids -contains $counterPid) {
          $values += [double]$counter.CookedValue
        }
      }
    }
    if ($values.Count -gt 0) {
      $result.total = [double](($values | Measure-Object -Sum).Sum)
      $result.maxEngine = [double](($values | Measure-Object -Maximum).Maximum)
    }
  }
  catch {
    # GPU Engine counters are driver-dependent; null is an explicit unavailable result.
  }
  return $result
}

function Get-Percentile([double[]]$Values, [double]$P) {
  if (($null -eq $Values) -or ($Values.Count -eq 0)) { return $null }
  $sorted = @($Values | Sort-Object)
  $index = [Math]::Ceiling($P * $sorted.Count) - 1
  if ($index -lt 0) { $index = 0 }
  if ($index -ge $sorted.Count) { $index = $sorted.Count - 1 }
  return [double]$sorted[$index]
}

function Get-NumberOrZero($Value) {
  if ($null -eq $Value) { return 0.0 }
  return [double]$Value
}

$resolved = [System.IO.Path]::GetFullPath((Join-Path (Get-Location) $ExecutablePath))
if (-not (Test-Path $resolved)) { throw "Executable not found: $resolved" }
New-Item -ItemType Directory -Force -Path $OutputDirectory | Out-Null

$ownedProcess = $false
$process = $null
if ($Launch) {
  $process = Start-Process -FilePath $resolved -PassThru
  $ownedProcess = $true
}
else {
  $process = Get-Process -Name 'northpalace-my-pet' -ErrorAction SilentlyContinue | Select-Object -First 1
  if ($null -eq $process) { throw "northpalace-my-pet is not running; use -Launch or start it first" }
}

try {
  if ($WarmupSeconds -gt 0) { Start-Sleep -Seconds $WarmupSeconds }
  if ($process.HasExited) { throw "Lenvu exited during warmup with code $($process.ExitCode)" }

  $logicalProcessors = [Math]::Max(1, [Environment]::ProcessorCount)
  $samples = @()
  $previousCpu = $null
  $previousTime = $null
  $deadline = (Get-Date).AddSeconds([Math]::Max(1, $SampleSeconds))

  while ((Get-Date) -lt $deadline) {
    if ($process.HasExited) { throw "Lenvu exited during sampling with code $($process.ExitCode)" }

    $ids = @(Get-DescendantIds $process.Id)
    $members = @(Get-Process -Id $ids -ErrorAction SilentlyContinue)
    $now = Get-Date

    $cpuSeconds = ($members | Measure-Object -Property CPU -Sum).Sum
    if ($null -eq $cpuSeconds) { $cpuSeconds = 0.0 }
    $cpuPercent = $null
    if (($null -ne $previousCpu) -and ($null -ne $previousTime)) {
      $elapsed = ($now - $previousTime).TotalSeconds
      if ($elapsed -gt 0) {
        $cpuPercent = (($cpuSeconds - $previousCpu) / $elapsed / $logicalProcessors) * 100.0
        if ($cpuPercent -lt 0) { $cpuPercent = 0.0 }
      }
    }
    $previousCpu = [double]$cpuSeconds
    $previousTime = $now

    $workingSet = ($members | Measure-Object -Property WorkingSet64 -Sum).Sum
    if ($null -eq $workingSet) { $workingSet = 0 }
    $privateBytes = ($members | Measure-Object -Property PrivateMemorySize64 -Sum).Sum
    if ($null -eq $privateBytes) { $privateBytes = 0 }
    $gpu = Get-GpuSample $ids

    $sampleRow = [ordered]@{
      timestamp = $now.ToUniversalTime().ToString('o')
      processCount = $members.Count
      cpuPercent = $cpuPercent
      workingSetBytes = [int64]$workingSet
      privateBytes = [int64]$privateBytes
      gpuEngineTotalPercent = $gpu.total
      gpuMaxEnginePercent = $gpu.maxEngine
    }
    $samples += [pscustomobject]$sampleRow
    Start-Sleep -Milliseconds ([Math]::Max(250, $IntervalMs))
  }

  $os = Get-CimInstance Win32_OperatingSystem
  $cpu = Get-CimInstance Win32_Processor | Select-Object -First 1
  $gpus = @(Get-CimInstance Win32_VideoController | Select-Object -ExpandProperty Name)
  $hash = (Get-FileHash $resolved -Algorithm SHA256).Hash

  $cpuValues = @($samples | Where-Object { $null -ne $_.cpuPercent } | ForEach-Object { [double]$_.cpuPercent })
  $workingValues = @($samples | ForEach-Object { [double]$_.workingSetBytes })
  $privateValues = @($samples | ForEach-Object { [double]$_.privateBytes })
  $gpuValues = @($samples | Where-Object { $null -ne $_.gpuEngineTotalPercent } | ForEach-Object { [double]$_.gpuEngineTotalPercent })

  $cpuAverage = $null
  $cpuMax = $null
  if ($cpuValues.Count -gt 0) {
    $cpuAverage = [double](($cpuValues | Measure-Object -Average).Average)
    $cpuMax = [double](($cpuValues | Measure-Object -Maximum).Maximum)
  }
  $workingAverage = $null
  if ($workingValues.Count -gt 0) { $workingAverage = [int64](($workingValues | Measure-Object -Average).Average) }
  $privateAverage = $null
  if ($privateValues.Count -gt 0) { $privateAverage = [int64](($privateValues | Measure-Object -Average).Average) }
  $gpuAverage = $null
  if ($gpuValues.Count -gt 0) { $gpuAverage = [double](($gpuValues | Measure-Object -Average).Average) }

  $summary = [ordered]@{
    cpuAveragePercent = $cpuAverage
    cpuP95Percent = Get-Percentile $cpuValues 0.95
    cpuMaxPercent = $cpuMax
    workingSetAverageBytes = $workingAverage
    workingSetP95Bytes = Get-Percentile $workingValues 0.95
    privateAverageBytes = $privateAverage
    gpuAverageEngineTotalPercent = $gpuAverage
    gpuP95EngineTotalPercent = Get-Percentile $gpuValues 0.95
  }

  $result = [ordered]@{
    schemaVersion = 1
    scenario = $Scenario
    recordedAtUtc = (Get-Date).ToUniversalTime().ToString('o')
    executable = [ordered]@{ path = $resolved; sha256 = $hash }
    system = [ordered]@{
      os = $os.Caption
      osVersion = $os.Version
      cpu = $cpu.Name
      logicalProcessors = $logicalProcessors
      totalVisibleMemoryBytes = [int64]$os.TotalVisibleMemorySize * 1024
      gpu = $gpus
    }
    sampling = [ordered]@{
      warmupSeconds = $WarmupSeconds
      sampleSeconds = $SampleSeconds
      intervalMs = $IntervalMs
      sampleCount = $samples.Count
    }
    summary = $summary
    samples = $samples
  }

  $stamp = (Get-Date).ToString('yyyyMMdd-HHmmss')
  $safeScenario = ($Scenario -replace '[^A-Za-z0-9_-]', '_')
  $base = Join-Path $OutputDirectory "lenvu-$safeScenario-$stamp"
  $jsonPath = "$base.json"
  $mdPath = "$base.md"
  $result | ConvertTo-Json -Depth 8 | Set-Content $jsonPath -Encoding utf8

  $mb = 1MB
  $cpuAverageText = [Math]::Round((Get-NumberOrZero $summary.cpuAveragePercent), 2)
  $cpuP95Text = [Math]::Round((Get-NumberOrZero $summary.cpuP95Percent), 2)
  $cpuMaxText = [Math]::Round((Get-NumberOrZero $summary.cpuMaxPercent), 2)
  $workingAverageText = [Math]::Round(((Get-NumberOrZero $summary.workingSetAverageBytes) / $mb), 1)
  $workingP95Text = [Math]::Round(((Get-NumberOrZero $summary.workingSetP95Bytes) / $mb), 1)
  $privateAverageText = [Math]::Round(((Get-NumberOrZero $summary.privateAverageBytes) / $mb), 1)
  $gpuAverageText = [Math]::Round((Get-NumberOrZero $summary.gpuAverageEngineTotalPercent), 2)
  $gpuP95Text = [Math]::Round((Get-NumberOrZero $summary.gpuP95EngineTotalPercent), 2)

  $markdown = @(
    '# Lenvu target-machine baseline',
    '',
    "- Scenario: $Scenario",
    "- Recorded UTC: $($result.recordedAtUtc)",
    "- Executable SHA-256: $hash",
    "- OS: $($os.Caption) $($os.Version)",
    "- CPU: $($cpu.Name)",
    "- GPU: $($gpus -join ', ')",
    "- Total visible RAM: $([Math]::Round(($result.system.totalVisibleMemoryBytes / $mb), 1)) MB",
    '',
    '| Metric | Result |',
    '|---|---:|',
    "| CPU average | $cpuAverageText % |",
    "| CPU p95 | $cpuP95Text % |",
    "| CPU max | $cpuMaxText % |",
    "| Working set average | $workingAverageText MB |",
    "| Working set p95 | $workingP95Text MB |",
    "| Private bytes average | $privateAverageText MB |",
    "| GPU engine-total average* | $gpuAverageText % |",
    "| GPU engine-total p95* | $gpuP95Text % |",
    '',
    '* GPU counters are best-effort Windows GPU Engine counters and can be unavailable on some drivers. The JSON retains null when unavailable.',
    '',
    "Raw samples: $jsonPath"
  )
  $markdown | Set-Content $mdPath -Encoding utf8
  Write-Host "Wrote $jsonPath"
  Write-Host "Wrote $mdPath"
}
finally {
  if ($ownedProcess -and ($null -ne $process) -and (-not $process.HasExited)) {
    Stop-Process -Id $process.Id -Force -ErrorAction SilentlyContinue
    Wait-Process -Id $process.Id -ErrorAction SilentlyContinue
  }
}
'''

path = ROOT / "scripts/measure-target-baseline.ps1"
path.write_text(script, encoding="utf-8", newline="\n")
print("Final Clippy and Windows PowerShell 5.1 gate fixes applied.")
