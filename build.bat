@echo off
REM Production deployment script for Censor (Windows)
REM Run this to prepare the application for release

setlocal enabledelayedexpansion

echo.
echo 🔨 Building Censor for production...
echo.

REM Check requirements
where cargo >nul 2>&1
if %errorlevel% neq 0 (
    echo Rust/Cargo not found
    exit /b 1
)

where npm >nul 2>&1
if %errorlevel% neq 0 (
    echo Node.js/npm not found
    exit /b 1
)

REM Build frontend
echo 📦 Building Vue.js frontend...
call npm run build

if %errorlevel% neq 0 (
    echo Frontend build failed
    exit /b 1
)

REM Build Tauri application
echo 🚀 Building Tauri application...
call npm run tauri:build

if %errorlevel% neq 0 (
    echo Tauri build failed
    exit /b 1
)

echo.
echo ✅ Build complete!
echo.
echo Output locations:
echo   Windows: src-tauri\target\release\censor.exe
echo.
echo Next steps:
echo   1. Test the built application
echo   2. Create installer
echo   3. Push to release repository
echo.

endlocal
