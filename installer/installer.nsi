; SysMon Installer Script
!include "MUI2.nsh"

; General
Name "SysMon"
OutFile "SysMon-Setup.exe"
InstallDir "$LOCALAPPDATA\SysMon"
InstallDirRegKey HKCU "Software\SysMon" "InstallDir"
RequestExecutionLevel admin

; Interface
!define MUI_ABORTWARNING
!define MUI_ICON "${NSISDIR}\Contrib\Graphics\Icons\modern-install.ico"

; Pages
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH

!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES

; Languages
!insertmacro MUI_LANGUAGE "TraditionalChinese"
!insertmacro MUI_LANGUAGE "English"

Section "Install"
    SetOutPath "$INSTDIR"
    
    ; Files
    File "target\release\sysmon.exe"
    File "config.toml"
    
    ; Create uninstaller
    WriteUninstaller "$INSTDIR\Uninstall.exe"
    
    ; Registry
    WriteRegStr HKCU "Software\SysMon" "InstallDir" "$INSTDIR"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\SysMon" "DisplayName" "SysMon"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\SysMon" "UninstallString" '"$INSTDIR\Uninstall.exe"'
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\SysMon" "InstallLocation" "$INSTDIR"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\SysMon" "DisplayVersion" "0.1.8"
    WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\SysMon" "Publisher" "SysMon"
    
    ; Start Menu
    CreateDirectory "$SMPROGRAMS\SysMon"
    CreateShortCut "$SMPROGRAMS\SysMon\SysMon.lnk" "$INSTDIR\sysmon.exe"
    CreateShortCut "$SMPROGRAMS\SysMon\Uninstall.lnk" "$INSTDIR\Uninstall.exe"
    
    ; Startup
    WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Run" "SysMon" "$INSTDIR\sysmon.exe"
    
SectionEnd

Section "Uninstall"
    ; Files
    Delete "$INSTDIR\sysmon.exe"
    Delete "$INSTDIR\config.toml"
    Delete "$INSTDIR\Uninstall.exe"
    RMDir "$INSTDIR"
    
    ; Registry
    DeleteRegValue HKCU "Software\Microsoft\Windows\CurrentVersion\Run" "SysMon"
    DeleteRegKey HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\SysMon"
    DeleteRegKey HKCU "Software\SysMon"
    
    ; Start Menu
    Delete "$SMPROGRAMS\SysMon\SysMon.lnk"
    Delete "$SMPROGRAMS\SysMon\Uninstall.lnk"
    RMDir "$SMPROGRAMS\SysMon"
    
SectionEnd
