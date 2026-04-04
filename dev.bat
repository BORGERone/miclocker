@echo off
title MicLocker - Dev Mode
cd /d "%~dp0"

echo Starting MicLocker in development mode...
echo.

call npm run tauri dev
