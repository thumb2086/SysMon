@echo off
setlocal

echo ====================================
echo SysMon Installer v0.4.0
echo ====================================
echo.

net session >nul 2>&1
if %errorlevel% neq 0 (
    echo Please run as Administrator!
    pause
    exit /b 1
)

set INSTALL_DIR=%LOCALAPPDATA%\SysMon

if not exist "%INSTALL_DIR%" mkdir "%INSTALL_DIR%"

echo Installing SysMon...
copy /Y "%~dp0sysmon.exe" "%INSTALL_DIR%\"

echo Creating shortcuts...
if not exist "%APPDATA%\Microsoft\Windows\Start Menu\Programs\SysMon" mkdir "%APPDATA%\Microsoft\Windows\Start Menu\Programs\SysMon"

echo Set oWS = WScript.CreateObject("WScript.Shell") > "%TEMP%\shortcut.vbs"
echo Set oLink = oWS.CreateShortcut("%APPDATA%\Microsoft\Windows\Start Menu\Programs\SysMon\SysMon.lnk") >> "%TEMP%\shortcut.vbs"
echo oLink.TargetPath = "%INSTALL_DIR%\sysmon.exe" >> "%TEMP%\shortcut.vbs"
echo oLink.WorkingDirectory = "%INSTALL_DIR%" >> "%TEMP%\shortcut.vbs"
echo oLink.Save >> "%TEMP%\shortcut.vbs"

cscript //nologo "%TEMP%\shortcut.vbs"
del "%TEMP%\shortcut.vbs"

setx PATH "%PATH%;%INSTALL_DIR%" >nul 2>&1

echo.
echo ====================================
echo Installation Complete!
echo ====================================
echo.
echo SysMon has been installed to: %INSTALL_DIR%
echo.
echo Features:
echo - CPU/GPU/Memory monitoring
echo - Network traffic tracking
echo - Daily/Monthly limits
echo - Auto-start with Windows
echo - System tray support
echo - Chinese/English language
echo.
pause
