param()

$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest
if ($env:OS -ne 'Windows_NT') {
    throw 'Run this script in Windows Developer PowerShell for Visual Studio.'
}
$sourceRoot = Split-Path -Parent $PSScriptRoot
$cargoBin = Join-Path $env:USERPROFILE '.cargo/bin'
$portablePerlBin = Join-Path $sourceRoot '.cache/perl-portable/perl/bin'
foreach ($toolBin in @($cargoBin, $portablePerlBin)) {
    if (Test-Path -LiteralPath $toolBin) {
        $env:PATH = "$toolBin;$env:PATH"
    }
}
if (-not (Get-Command cl -ErrorAction SilentlyContinue)) {
    $vswhere = Join-Path ${env:ProgramFiles(x86)} 'Microsoft Visual Studio/Installer/vswhere.exe'
    if (Test-Path -LiteralPath $vswhere) {
        $vsRoot = & $vswhere -latest -products '*' -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath
        if ($vsRoot) {
            Import-Module (Join-Path $vsRoot 'Common7/Tools/Microsoft.VisualStudio.DevShell.dll')
            Enter-VsDevShell -VsInstallPath $vsRoot -SkipAutomaticLocation -DevCmdArguments '-arch=amd64 -host_arch=amd64' | Out-Null
            $cmakeBin = Join-Path $vsRoot 'Common7/IDE/CommonExtensions/Microsoft/CMake/CMake/bin'
            if (Test-Path -LiteralPath $cmakeBin) {
                $env:PATH = "$cmakeBin;$env:PATH"
            }
        }
    }
}
foreach ($tool in @('cargo', 'rustc', 'cl', 'cmake', 'perl')) {
    if (-not (Get-Command $tool -ErrorAction SilentlyContinue)) {
        throw "Missing '$tool'. Install Rust, C++ Build Tools/Windows SDK, CMake and Perl; then use Developer PowerShell."
    }
}

function Invoke-Cargo {
    param([string[]]$Arguments)
    Write-Output "Running cargo $($Arguments -join ' ')"
    & cargo @Arguments
    if ($LASTEXITCODE -ne 0) {
        throw "cargo $($Arguments -join ' ') failed (exit $LASTEXITCODE)."
    }
}

Push-Location -LiteralPath $sourceRoot
$priorRustdocFlags = $env:RUSTDOCFLAGS
try {
    Invoke-Cargo -Arguments @('fmt', '--all', '--check')
    Invoke-Cargo -Arguments @('clippy', '--locked', '--all-targets', '--', '-D', 'warnings')
    Invoke-Cargo -Arguments @('clippy', '--locked', '--all-targets', '--all-features', '--', '-D', 'warnings')
    Invoke-Cargo -Arguments @('test', '--locked', '--all-targets')
    Invoke-Cargo -Arguments @('test', '--locked', '--all-targets', '--all-features')
    $env:RUSTDOCFLAGS = '-D warnings'
    Invoke-Cargo -Arguments @('doc', '--locked', '--all-features', '--no-deps')
    Invoke-Cargo -Arguments @('build', '--locked', '--release', '--target', 'x86_64-pc-windows-msvc')
    $outputDir = Join-Path $sourceRoot 'dist/windows-compat'
    New-Item -ItemType Directory -Force -Path $outputDir | Out-Null
    Copy-Item -LiteralPath 'target/x86_64-pc-windows-msvc/release/zapfast.exe' -Destination $outputDir
    Copy-Item -LiteralPath 'LICENSE' -Destination $outputDir
    Copy-Item -LiteralPath 'THIRD-PARTY-NOTICES.md' -Destination $outputDir
    Copy-Item -LiteralPath 'docs/windows-compatibility.md' -Destination $outputDir
    Copy-Item -LiteralPath 'docs/pairing-recovery.md' -Destination $outputDir
    Copy-Item -LiteralPath 'docs/performance-windows.md' -Destination $outputDir
    Copy-Item -LiteralPath 'packaging/windows-compat-README.md' -Destination (Join-Path $outputDir 'README.md')
    Copy-Item -LiteralPath 'scripts/start-software-rendering.cmd' -Destination $outputDir
    $hash = (Get-FileHash -Algorithm SHA256 -LiteralPath (Join-Path $outputDir 'zapfast.exe')).Hash.ToLowerInvariant()
    "$hash  zapfast.exe" | Set-Content -LiteralPath (Join-Path $outputDir 'SHA256.txt')
    Write-Host "Build and automated checks passed. Output: $outputDir. Display and account tests remain required."
} finally {
    $env:RUSTDOCFLAGS = $priorRustdocFlags
    Pop-Location
}
