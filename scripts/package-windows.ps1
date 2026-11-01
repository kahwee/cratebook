param(
    [string]$TargetDir = "target"
)

$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest
Push-Location (Join-Path $PSScriptRoot "..")
try {
    # Shells opened through WSL may not inherit rustup's user PATH entry.
    $cargoCommand = Get-Command cargo -ErrorAction SilentlyContinue
    $rustBin = Join-Path $env:USERPROFILE ".cargo/bin"
    $cargoPath = if ($cargoCommand) { $cargoCommand.Source } else { Join-Path $rustBin "cargo.exe" }
    $rustcCommand = Get-Command rustc -ErrorAction SilentlyContinue
    $rustcPath = if ($rustcCommand) { $rustcCommand.Source } else { Join-Path $rustBin "rustc.exe" }
    $hostInfo = & $rustcPath -vV
    if ($LASTEXITCODE -ne 0) { throw "Unable to inspect the Rust toolchain" }
    $hostLine = $hostInfo | Where-Object { $_ -match '^host: ' }
    if ($hostLine -ne 'host: x86_64-pc-windows-msvc') {
        throw "This package script requires the x86_64-pc-windows-msvc toolchain"
    }
    & $cargoPath build --locked --release --bin cratebook --target-dir $TargetDir
    if ($LASTEXITCODE -ne 0) { throw "Windows release build failed" }
    $versionLine = Get-Content Cargo.toml | Where-Object { $_ -match '^version = "' } | Select-Object -First 1
    $version = $versionLine.Split('"')[1]
    $name = "cratebook-$version-windows-x86_64"
    $stage = Join-Path "dist" $name
    New-Item -ItemType Directory -Force -Path $stage | Out-Null
    Copy-Item -LiteralPath (Join-Path $TargetDir "release/cratebook.exe") -Destination $stage
    Copy-Item -LiteralPath LICENSE, THIRD_PARTY.md, README.md, Cargo.lock -Destination $stage
    $docs = Join-Path $stage "docs"
    New-Item -ItemType Directory -Force -Path $docs | Out-Null
    Copy-Item -LiteralPath docs/windows-testing.md -Destination $docs
    $licenses = Join-Path $stage "licenses"
    New-Item -ItemType Directory -Force -Path $licenses | Out-Null
    Copy-Item -LiteralPath assets/OFL.txt, vendor/gpui-pre-web/LICENSE-APACHE -Destination $licenses
    $archive = Join-Path "dist" "$name.zip"
    Compress-Archive -Path "$stage/*" -DestinationPath $archive -Force
    $stream = [System.IO.File]::OpenRead((Resolve-Path -LiteralPath $archive).Path)
    $sha256 = [System.Security.Cryptography.SHA256]::Create()
    try {
        $hash = [BitConverter]::ToString($sha256.ComputeHash($stream)).Replace('-', '').ToLowerInvariant()
    } finally {
        $stream.Dispose()
        $sha256.Dispose()
    }
    "$hash  $name.zip" | Set-Content -Encoding ascii -LiteralPath "$archive.sha256"
    Write-Output "Created $archive"
} finally {
    Pop-Location
}
