from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def patch(path: str, old: str, new: str, count: int = 1) -> None:
    target = ROOT / path
    text = target.read_text(encoding="utf-8")
    actual = text.count(old)
    if actual != count:
        raise RuntimeError(f"{path}: expected {count} match(es), found {actual}: {old!r}")
    target.write_text(text.replace(old, new), encoding="utf-8", newline="\n")


# MemoryOrigin only needs stable deserialization today; writes are explicit SQL literals protected
# by CHECK constraints. Remove the unused formatter rather than suppressing Clippy dead-code.
patch(
    "src-tauri/src/domain/memory.rs",
    '''    pub fn as_str(self) -> &'static str {
        match self {
            Self::Manual => "manual",
            Self::Automatic => "automatic",
        }
    }

''',
    "",
)
patch(
    "src-tauri/src/domain/memory.rs",
    '''            assert_eq!(origin.as_str(), raw);
            assert_eq!(MemoryOrigin::from_str(raw), Some(origin));
''',
    '''            assert_eq!(MemoryOrigin::from_str(raw), Some(origin));
''',
)

# Windows PowerShell variables are case-insensitive, so `$errors` aliases the automatic `$Error`
# collection. Use dedicated parser variables in the permanent Windows CI parse gate.
patch(
    ".github/workflows/windows-ci.yml",
    '''          $tokens = $null
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
''',
    '''          $parseTokens = $null
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
''',
)

print("Applied final Clippy and Windows PowerShell gate fixups.")
