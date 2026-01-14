# Get LibRaw Windows Libraries Script
$ErrorActionPreference = "Stop"

Write-Host "QuakImages - Windows Library Setup" -ForegroundColor Cyan
Write-Host ""

$PROJECT_ROOT = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$DEST_PATH = Join-Path $PROJECT_ROOT "src-tauri\vendor\libraw-sys\libs\windows\x64"

Write-Host "Project: $PROJECT_ROOT" -ForegroundColor Cyan
Write-Host "Destination: $DEST_PATH" -ForegroundColor Cyan
Write-Host ""

$libFiles = @("raw_r.lib", "lcms2.lib", "jpeg.lib", "zlib.lib")
$allExist = $true

foreach ($libFile in $libFiles) {
    $libPath = Join-Path $DEST_PATH $libFile
    if (Test-Path $libPath) {
        Write-Host "  [OK] $libFile exists" -ForegroundColor Green
    } else {
        $allExist = $false
    }
}

if ($allExist) {
    Write-Host ""
    Write-Host "[OK] All libraries already in project!" -ForegroundColor Green
    Write-Host "You can compile with: bun run tauri dev" -ForegroundColor Cyan
    exit 0
}

Write-Host ""
Write-Host "Missing libraries. Getting them..." -ForegroundColor Yellow
Write-Host ""

$VCPKG_TEMP = Join-Path $env:TEMP "vcpkg_libraw_temp"

if (Test-Path $VCPKG_TEMP) {
    Remove-Item -Recurse -Force $VCPKG_TEMP
}

Write-Host "Cloning vcpkg..." -ForegroundColor Yellow
git clone --depth 1 https://github.com/Microsoft/vcpkg.git $VCPKG_TEMP 2>&1 | Out-Null

if ($LASTEXITCODE -ne 0) {
    Write-Host "[ERROR] Failed to clone vcpkg. Is git installed?" -ForegroundColor Red
    exit 1
}

Write-Host "[OK] vcpkg cloned" -ForegroundColor Green

Write-Host "Bootstrapping vcpkg..." -ForegroundColor Yellow
Push-Location $VCPKG_TEMP
& .\bootstrap-vcpkg.bat | Out-Null

if ($LASTEXITCODE -ne 0) {
    Write-Host "[ERROR] Failed to bootstrap vcpkg" -ForegroundColor Red
    Pop-Location
    exit 1
}

Write-Host "[OK] vcpkg ready" -ForegroundColor Green
Write-Host ""
Write-Host "Installing libraries (5-10 minutes)..." -ForegroundColor Yellow

& .\vcpkg.exe install libraw:x64-windows-static lcms:x64-windows-static libjpeg-turbo:x64-windows-static zlib:x64-windows-static

if ($LASTEXITCODE -ne 0) {
    Write-Host "[ERROR] Failed to install libraries" -ForegroundColor Red
    Pop-Location
    exit 1
}

Pop-Location

Write-Host ""
Write-Host "[OK] Libraries compiled" -ForegroundColor Green
Write-Host ""
Write-Host "Copying libraries to project..." -ForegroundColor Yellow

$SOURCE_LIB_PATH = Join-Path $VCPKG_TEMP "installed\x64-windows-static\lib"

if (!(Test-Path $DEST_PATH)) {
    New-Item -ItemType Directory -Path $DEST_PATH -Force | Out-Null
}

$copiedCount = 0
foreach ($libFile in $libFiles) {
    $sourcePath = Join-Path $SOURCE_LIB_PATH $libFile
    $destPath = Join-Path $DEST_PATH $libFile
    
    if (Test-Path $sourcePath) {
        Copy-Item $sourcePath -Destination $destPath -Force
        $fileSize = (Get-Item $destPath).Length / 1MB
        $fileSizeRounded = [math]::Round($fileSize, 2)
        Write-Host "  [OK] $libFile copied ($fileSizeRounded MB)" -ForegroundColor Green
        $copiedCount++
    } else {
        Write-Host "  [ERROR] $libFile not found" -ForegroundColor Red
    }
}

Write-Host ""

if ($copiedCount -eq $libFiles.Count) {
    Write-Host "[SUCCESS] All libraries copied!" -ForegroundColor Green
} else {
    Write-Host "[WARNING] Only $copiedCount of $($libFiles.Count) libraries copied" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "Cleaning up..." -ForegroundColor Yellow
Remove-Item -Recurse -Force $VCPKG_TEMP

Write-Host "[OK] Done!" -ForegroundColor Green
Write-Host ""
Write-Host "You can now compile: bun run tauri dev" -ForegroundColor Cyan
Write-Host ""
Write-Host "Consider committing the libraries to Git" -ForegroundColor Yellow
Write-Host "so other developers don't need to run this script." -ForegroundColor Yellow
