@echo off
title MicLocker - Build
cd /d "%~dp0"

echo ========================================
echo   MicLocker - Building...
echo ========================================
echo.

call npm run tauri build

if %errorlevel% neq 0 (
    echo.
    echo BUILD FAILED!
    pause
    exit /b 1
)

echo.
echo ========================================
echo   Build successful!
echo ========================================
echo.
echo MSI:    src-tauri\target\release\bundle\msi\MicLocker_1.0.0_x64_en-US.msi
echo NSIS:   src-tauri\target\release\bundle\nsis\MicLocker_1.0.0_x64-setup.exe
echo EXE:    src-tauri\target\release\miclocker.exe
echo.
pause
