param(
    [string]$RepositoryRoot = (Split-Path -Parent $PSScriptRoot)
)

$ErrorActionPreference = 'Stop'
$sourceRoots = @(
    (Join-Path $RepositoryRoot 'src'),
    (Join-Path $RepositoryRoot 'crates/pong_core/src'),
    (Join-Path $RepositoryRoot 'crates/pong_core/tests')
)
$files = Get-ChildItem -LiteralPath $sourceRoots -Filter '*.rs' -File -Recurse
# A lightweight check for this repository's explicit, rustfmt-formatted declarations.
# Rust's compiler and crate manifests remain responsible for dependency visibility.
$declarationPattern = '(?m)^\s*(?:pub(?:\([^)]*\))?\s+)?(?:struct|enum|trait)\s+(\w+)'
$violations = @(
    foreach ($file in $files) {
        $source = [IO.File]::ReadAllText($file.FullName)
        $declarations = [regex]::Matches($source, $declarationPattern)
        if ($declarations.Count -gt 1) {
            $names = ($declarations | ForEach-Object { $_.Groups[1].Value }) -join ', '
            "$($file.FullName): put each type or trait in its own file ($names)."
        }
        if ($file.Name -eq 'mod.rs' -and $declarations.Count -gt 0) {
            "$($file.FullName): keep type definitions in their own files, outside mod.rs."
        }
    }
)
if ($violations.Count -gt 0) {
    throw ($violations -join [Environment]::NewLine)
}
Write-Output "Source layout checked: $($files.Count) Rust files, at most one type or trait per file."
