from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
path = ROOT / ".github/workflows/windows-ci.yml"
text = path.read_text(encoding="utf-8")
old = '''          $tokens = $null
          $errors = $null
          [System.Management.Automation.Language.Parser]::ParseFile(
            (Resolve-Path 'scripts/measure-target-baseline.ps1'),
            [ref]$tokens,
            [ref]$errors
          ) | Out-Null
          if ($errors.Count -gt 0) {
            $errors | ForEach-Object { Write-Error $_.Message }
            exit 1
          }
'''
new = '''          $parseTokens = $null
          $parseErrors = $null
          [System.Management.Automation.Language.Parser]::ParseFile(
            (Resolve-Path 'scripts/measure-target-baseline.ps1'),
            [ref]$parseTokens,
            [ref]$parseErrors
          ) | Out-Null
          if (@($parseErrors).Count -gt 0) {
            $parseErrors | ForEach-Object { Write-Error $_.Message }
            exit 1
          }
'''
if text.count(old) != 1:
    raise RuntimeError(f"windows-ci parser wrapper changed: {text.count(old)} matches")
path.write_text(text.replace(old, new), encoding="utf-8", newline="\n")
print("Fixed permanent Windows CI PowerShell parser wrapper.")
