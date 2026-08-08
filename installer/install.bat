@echo off
setlocal

echo ====================================
echo SysMon Installer
echo ====================================
echo.

:: Check for admin rights
net session >nul 2>&1
if %errorlevel% neq 0 (
    echo Please run as Administrator!
    echo Right-click and select "Run as administrator"
    pause
    exit /b 1
)

:: Set installation directory
set INSTALL_DIR=%LOCALAPPDATA%\SysMon

:: Create installation directory
if not exist "%INSTALL_DIR%" mkdir "%INSTALL_DIR%"

:: Copy files
echo Installing SysMon...
copy /Y "%~dp0sysmon.exe" "%INSTALL_DIR%\"
copy /Y "%~dp0config.toml" "%INSTALL_DIR%\"

:: Create Start Menu shortcuts
echo Creating shortcuts...
if not exist "%APPDATA%\Microsoft\Windows\Start Menu\Programs\SysMon" mkdir "%APPDATA%\Microsoft\Windows\Start Menu\Programs\SysMon"

echo Set oWS = WScript.CreateObject("WScript.Shell") > "%TEMP%\shortcut.vbs"
echo Set oLink = oWS.CreateShortcut("%APPDATA%\Microsoft\Windows\Start Menu\Programs\SysMon\SysMon.lnk") >> "%TEMP%\shortcut.vbs"
echo oLink.TargetPath = "%INSTALL_DIR%\sysmon.exe" >> "%TEMP%\shortcut.vbs"
echo oLink.WorkingDirectory = "%INSTALL_DIR%" >> "%TEMP%\shortcut.vbs"
echo oLink.Save >> "%TEMP%\shortcut.vbs"

cscript //nologo "%TEMP%\shortcut.vbs"
del "%TEMP%\shortcut.vbs"

:: Add to PATH
setx PATH "%PATH%;%INSTALL_DIR%" >nul 2>&1

echo.
echo ====================================
echo Installation Complete!
echo ====================================
echo.
echo SysMon has been installed to: %INSTALL_DIR%
echo.
echo You can now run SysMon from:
echo - Start Menu
echo - Command line: sysmon
echo.
pause
