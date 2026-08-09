; SysMon Installer
!include "MUI2.nsh"

Name "SysMon"
OutFile "SysMon-Setup.exe"
InstallDir "$LOCALAPPDATA\SysMon"
InstallDirRegKey HKCU "Software\SysMon" "InstallDir"
RequestExecutionLevel admin

!define MUI_ABORTWARNING
!define MUI_ICON "${NSISDIR}\Contrib\Graphics\Icons\modern-install.ico"

!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH

!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES

!insertmacro MUI_LANGUAGE "TraditionalChinese"
!insertmacro MUI_LANGUAGE "English"

Section "Install"
    SetOutPath "$INSTDIR"
    File "target\release\sysmon.exe"
    
    WriteUninstaller "$INSTDIR\Uninstall.exe"
    WriteRegStr HKCU "Software\SysMon" "InstallDir" "$INSTDIR"
    
    CreateDirectory "$SMPROGRAMS\SysMon"
    CreateShortCut "$SMPROGRAMS\SysMon\SysMon.lnk" "$INSTDIR\sysmon.exe"
    CreateShortCut "$SMPROGRAMS\SysMon\Uninstall.lnk" "$INSTDIR\Uninstall.exe"
    
    WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Run" "SysMon" "$INSTDIR\sysmon.exe"
SectionEnd

Section "Uninstall"
    Delete "$INSTDIR\sysmon.exe"
    Delete "$INSTDIR\Uninstall.exe"
    RMDir "$INSTDIR"
    
    DeleteRegValue HKCU "Software\Microsoft\Windows\CurrentVersion\Run" "SysMon"
    DeleteRegKey HKCU "Software\SysMon"
    
    Delete "$SMPROGRAMS\SysMon\SysMon.lnk"
    Delete "$SMPROGRAMS\SysMon\Uninstall.lnk"
    RMDir "$SMPROGRAMS\SysMon"
SectionEnd
