param(
    [string]$RepositoryRoot = (Split-Path -Parent $PSScriptRoot)
)

$ErrorActionPreference = 'Stop'
# This checks explicit Rust paths in this repository. Cargo and Rust visibility
# enforce the crate boundary and private module access; this is not a Rust parser.
$rules = @(
    @{
        Path = 'crates/pong_core/src/domain'
        Forbidden = '\bcrate::(?!domain::)|\b(?:application|bevy|avian2d|pong)::'
        Reason = 'Domain code may depend only on domain code and the Rust standard library.'
    },
    @{
        Path = 'crates/pong_core/src/application'
        Forbidden = '\bcrate::(?!(?:application|domain)::)|\b(?:bevy|avian2d|pong)::'
        Reason = 'Application code may depend on its own ports and the domain, not engine adapters.'
    },
    @{
        Path = 'src/game/infrastructure'
        Forbidden = '\b(?:presentation|composition)::'
        Reason = 'Infrastructure must not depend on presentation or composition.'
    },
    @{
        Path = 'src/game/presentation'
        Forbidden = '\b(?:infrastructure|composition|avian2d)\s*(?:::|[,;}]|\bas\b)'
        Reason = 'Presentation may use the core and Bevy UI, not infrastructure, Avian, or composition.'
    }
)
$violations = @(
    foreach ($rule in $rules) {
        $directory = Join-Path $RepositoryRoot $rule.Path
        if (-not (Test-Path -LiteralPath $directory -PathType Container)) {
            throw "Missing architecture layer: $directory"
        }
        foreach ($file in Get-ChildItem -LiteralPath $directory -Filter '*.rs' -Recurse -File) {
            if ($file.Name -eq 'tests.rs') { continue }
            $lineNumber = 0
            foreach ($line in [IO.File]::ReadAllLines($file.FullName)) {
                $lineNumber++
                $code = ($line -split '//', 2)[0]
                if ($code -match $rule.Forbidden) {
                    "$($file.FullName):$lineNumber $($rule.Reason)"
                }
            }
        }
    }
)
if ($violations.Count -gt 0) {
    throw ($violations -join [Environment]::NewLine)
}
Write-Output 'Architecture paths checked: domain, application, infrastructure, and presentation.'
