; ============================================================================
; instfiles.nsh — File installation, registry, and uninstaller creation
; ============================================================================

!ifndef INSTFILES_NSH
!define INSTFILES_NSH

!include "LogicLib.nsh"

; Insert the standard install-files progress page
!insertmacro MUI_PAGE_INSTFILES

; ============================================================================
; Main install section
; ============================================================================
Section "Mush Shell" SecMain
    SectionIn RO ; required — cannot be deselected

    ; --- Create install directories ---
    SetOutPath "$INSTDIR"
    CreateDirectory "$INSTDIR\bin"

    ; --- Copy binaries ---
    SetOutPath "$INSTDIR\bin"
    File "${DIST_DIR}\mush.exe"
    File "${DIST_DIR}\cat.exe"
    File "${DIST_DIR}\cp.exe"
    File "${DIST_DIR}\cut.exe"
    File "${DIST_DIR}\date.exe"
    File "${DIST_DIR}\echo.exe"
    File "${DIST_DIR}\find.exe"
    File "${DIST_DIR}\grep.exe"
    File "${DIST_DIR}\head.exe"
    File "${DIST_DIR}\kill.exe"
    File "${DIST_DIR}\ls.exe"
    File "${DIST_DIR}\mkdir.exe"
    File "${DIST_DIR}\mv.exe"
    File "${DIST_DIR}\ps.exe"
    File "${DIST_DIR}\rm.exe"
    File "${DIST_DIR}\sed.exe"
    File "${DIST_DIR}\sleep.exe"
    File "${DIST_DIR}\sort.exe"
    File "${DIST_DIR}\tail.exe"
    File "${DIST_DIR}\touch.exe"
    File "${DIST_DIR}\uniq.exe"
    File "${DIST_DIR}\wc.exe"
    File "${DIST_DIR}\whoami.exe"

    ; --- Set permissions so mush can write config/db to install dir ---
    ; Grant the installing user full control over the install directory
    ; This allows mush.exe to create config.toml and .db on first run
    ${If} $InstallMode == "system"
        ; For system installs, give Users group modify access
        nsExec::ExecToLog 'icacls "$INSTDIR" /grant *S-1-5-32-545:(OI)(CI)M /T /Q'
        Pop $0 ; consume nsExec return value
    ${EndIf}
    ; For user installs, the user already owns the directory

    ; --- Write uninstaller ---
    SetOutPath "$INSTDIR"
    WriteUninstaller "$INSTDIR\uninstall.exe"

    ; --- Registry: Add/Remove Programs entry ---
    ${If} $InstallMode == "system"
        WriteRegStr HKLM "${UNINSTALL_KEY}" "DisplayName" "${APP_NAME}"
        WriteRegStr HKLM "${UNINSTALL_KEY}" "DisplayVersion" "${APP_VERSION}"
        WriteRegStr HKLM "${UNINSTALL_KEY}" "Publisher" "${APP_PUBLISHER}"
        WriteRegStr HKLM "${UNINSTALL_KEY}" "UninstallString" '"$INSTDIR\uninstall.exe"'
        WriteRegStr HKLM "${UNINSTALL_KEY}" "QuietUninstallString" '"$INSTDIR\uninstall.exe" /S'
        WriteRegStr HKLM "${UNINSTALL_KEY}" "InstallLocation" "$INSTDIR"
        WriteRegStr HKLM "${UNINSTALL_KEY}" "URLInfoAbout" "${APP_URL}"
        WriteRegDWORD HKLM "${UNINSTALL_KEY}" "NoModify" 1
        WriteRegDWORD HKLM "${UNINSTALL_KEY}" "NoRepair" 1
        ; Estimated size in KB
        ${GetSize} "$INSTDIR" "/S=0K" $0 $1 $2
        IntFmt $0 "0x%08X" $0
        WriteRegDWORD HKLM "${UNINSTALL_KEY}" "EstimatedSize" $0
    ${Else}
        WriteRegStr HKCU "${UNINSTALL_KEY}" "DisplayName" "${APP_NAME}"
        WriteRegStr HKCU "${UNINSTALL_KEY}" "DisplayVersion" "${APP_VERSION}"
        WriteRegStr HKCU "${UNINSTALL_KEY}" "Publisher" "${APP_PUBLISHER}"
        WriteRegStr HKCU "${UNINSTALL_KEY}" "UninstallString" '"$INSTDIR\uninstall.exe"'
        WriteRegStr HKCU "${UNINSTALL_KEY}" "QuietUninstallString" '"$INSTDIR\uninstall.exe" /S'
        WriteRegStr HKCU "${UNINSTALL_KEY}" "InstallLocation" "$INSTDIR"
        WriteRegStr HKCU "${UNINSTALL_KEY}" "URLInfoAbout" "${APP_URL}"
        WriteRegDWORD HKCU "${UNINSTALL_KEY}" "NoModify" 1
        WriteRegDWORD HKCU "${UNINSTALL_KEY}" "NoRepair" 1
        ${GetSize} "$INSTDIR" "/S=0K" $0 $1 $2
        IntFmt $0 "0x%08X" $0
        WriteRegDWORD HKCU "${UNINSTALL_KEY}" "EstimatedSize" $0
    ${EndIf}

    ; --- Store install mode for uninstaller ---
    ${If} $InstallMode == "system"
        WriteRegStr HKLM "${UNINSTALL_KEY}" "InstallMode" "$InstallMode"
    ${Else}
        WriteRegStr HKCU "${UNINSTALL_KEY}" "InstallMode" "$InstallMode"
    ${EndIf}

    ; --- Add to PATH ---
    ${If} $OptAddToPath == ${BST_CHECKED}
        StrCpy $R8 "$INSTDIR\bin"
        ${If} $InstallMode == "system"
            StrCpy $R9 "system"
        ${Else}
            StrCpy $R9 "user"
        ${EndIf}
        Call AddToPath
    ${EndIf}

    ; --- Create Start Menu shortcuts ---
    ${If} $OptStartMenu == ${BST_CHECKED}
        CreateDirectory "$SMPROGRAMS\${APP_NAME}"
        CreateShortCut "$SMPROGRAMS\${APP_NAME}\Mush.lnk" "$INSTDIR\bin\mush.exe" "" "" 0
        CreateShortCut "$SMPROGRAMS\${APP_NAME}\Uninstall Mush.lnk" "$INSTDIR\uninstall.exe" "" "" 0
    ${EndIf}

    ; --- Windows Terminal profile ---
    ${If} $OptWindowsTerminal == ${BST_CHECKED}
        Call AddWindowsTerminalProfile
    ${EndIf}

    ; --- Set as default shell ---
    ${If} $OptSetDefaultShell == ${BST_CHECKED}
        ${If} $InstallMode == "system"
            WriteRegStr HKLM "SYSTEM\CurrentControlSet\Control\Session Manager\Environment" "COMSPEC" "$INSTDIR\bin\mush.exe"
        ${Else}
            WriteRegStr HKCU "Environment" "COMSPEC" "$INSTDIR\bin\mush.exe"
        ${EndIf}
        SendMessage ${HWND_BROADCAST} ${WM_SETTINGCHANGE} 0 "STR:Environment" /TIMEOUT=5000
    ${EndIf}

    ; --- Install Bun runtime ---
    ${If} $OptInstallBun == ${BST_CHECKED}
        DetailPrint "Installing Bun runtime..."
        nsExec::ExecToLog 'powershell.exe -NoProfile -ExecutionPolicy Bypass -Command "irm bun.sh/install.ps1 | iex"'
        Pop $0
        ${If} $0 != 0
            DetailPrint "Warning: Bun installation may have failed (exit code: $0). Install manually from https://bun.sh"
        ${Else}
            DetailPrint "Bun runtime installed successfully."
        ${EndIf}
    ${EndIf}

SectionEnd

; ============================================================================
; Windows Terminal profile helper
; ============================================================================
Function AddWindowsTerminalProfile
    Push $0
    Push $1

    ; Try to find Windows Terminal settings.json
    ; Standard installation path
    StrCpy $0 "$LOCALAPPDATA\Packages\Microsoft.WindowsTerminal_8wekyb3d8bbwe\LocalState\settings.json"
    IfFileExists $0 _wt_found

    ; Preview version
    StrCpy $0 "$LOCALAPPDATA\Packages\Microsoft.WindowsTerminalPreview_8wekyb3d8bbwe\LocalState\settings.json"
    IfFileExists $0 _wt_found

    ; Not found — skip silently
    Goto _wt_done

    _wt_found:
    ; Write a helper script that adds the profile via PowerShell
    ; This avoids fragile JSON string manipulation in NSIS
    FileOpen $1 "$PLUGINSDIR\add_wt_profile.ps1" w
    FileWrite $1 '$$settingsPath = "$0"$\r$\n'
    FileWrite $1 '$$json = Get-Content $$settingsPath -Raw | ConvertFrom-Json$\r$\n'
    FileWrite $1 '$$guid = "{e5a83867-4f30-4a6e-b0ea-2a4b9c0a6f73}"$\r$\n'
    FileWrite $1 '$$existing = $$json.profiles.list | Where-Object { $$_.guid -eq $$guid }$\r$\n'
    FileWrite $1 'if (-not $$existing) {$\r$\n'
    FileWrite $1 '    $$profile = [PSCustomObject]@{$\r$\n'
    FileWrite $1 '        guid = $$guid$\r$\n'
    FileWrite $1 '        name = "Mush"$\r$\n'
    FileWrite $1 '        commandline = "$INSTDIR\bin\mush.exe"$\r$\n'
    FileWrite $1 '        hidden = $$false$\r$\n'
    FileWrite $1 '    }$\r$\n'
    FileWrite $1 '    $$json.profiles.list += $$profile$\r$\n'
    FileWrite $1 '    $$json | ConvertTo-Json -Depth 100 | Set-Content $$settingsPath -Encoding UTF8$\r$\n'
    FileWrite $1 '}$\r$\n'
    FileClose $1

    nsExec::ExecToLog 'powershell.exe -NoProfile -ExecutionPolicy Bypass -File "$PLUGINSDIR\add_wt_profile.ps1"'
    Pop $0 ; consume nsExec return value

    _wt_done:
    Pop $1
    Pop $0
FunctionEnd

; ============================================================================
; Uninstaller section
; ============================================================================
Section "Uninstall"
    ; --- Read install mode from registry ---
    ReadRegStr $InstallMode HKCU "${UNINSTALL_KEY}" "InstallMode"
    ${If} $InstallMode == ""
        ReadRegStr $InstallMode HKLM "${UNINSTALL_KEY}" "InstallMode"
    ${EndIf}
    ${If} $InstallMode == "system"
        SetShellVarContext all
    ${Else}
        SetShellVarContext current
    ${EndIf}

    ; --- Remove from PATH ---
    StrCpy $R8 "$INSTDIR\bin"
    ${If} $InstallMode == "system"
        StrCpy $R9 "system"
    ${Else}
        StrCpy $R9 "user"
    ${EndIf}
    Call un.RemoveFromPath

    ; --- Restore COMSPEC if we changed it ---
    ${If} $InstallMode == "system"
        ReadRegStr $0 HKLM "SYSTEM\CurrentControlSet\Control\Session Manager\Environment" "COMSPEC"
    ${Else}
        ReadRegStr $0 HKCU "Environment" "COMSPEC"
    ${EndIf}
    ${If} $0 == "$INSTDIR\bin\mush.exe"
        ${If} $InstallMode == "system"
            WriteRegStr HKLM "SYSTEM\CurrentControlSet\Control\Session Manager\Environment" "COMSPEC" "$WINDIR\system32\cmd.exe"
        ${Else}
            DeleteRegValue HKCU "Environment" "COMSPEC"
        ${EndIf}
        SendMessage ${HWND_BROADCAST} ${WM_SETTINGCHANGE} 0 "STR:Environment" /TIMEOUT=5000
    ${EndIf}

    ; --- Remove Windows Terminal profile ---
    Call un.RemoveWindowsTerminalProfile

    ; --- Remove Start Menu shortcuts ---
    RMDir /r "$SMPROGRAMS\${APP_NAME}"

    ; --- Remove installed files ---
    Delete "$INSTDIR\bin\mush.exe"
    Delete "$INSTDIR\bin\cat.exe"
    Delete "$INSTDIR\bin\cp.exe"
    Delete "$INSTDIR\bin\cut.exe"
    Delete "$INSTDIR\bin\date.exe"
    Delete "$INSTDIR\bin\echo.exe"
    Delete "$INSTDIR\bin\find.exe"
    Delete "$INSTDIR\bin\grep.exe"
    Delete "$INSTDIR\bin\head.exe"
    Delete "$INSTDIR\bin\kill.exe"
    Delete "$INSTDIR\bin\ls.exe"
    Delete "$INSTDIR\bin\mkdir.exe"
    Delete "$INSTDIR\bin\mv.exe"
    Delete "$INSTDIR\bin\ps.exe"
    Delete "$INSTDIR\bin\rm.exe"
    Delete "$INSTDIR\bin\sed.exe"
    Delete "$INSTDIR\bin\sleep.exe"
    Delete "$INSTDIR\bin\sort.exe"
    Delete "$INSTDIR\bin\tail.exe"
    Delete "$INSTDIR\bin\touch.exe"
    Delete "$INSTDIR\bin\uniq.exe"
    Delete "$INSTDIR\bin\wc.exe"
    Delete "$INSTDIR\bin\whoami.exe"
    RMDir "$INSTDIR\bin"

    ; Remove config and db if they exist (user data)
    Delete "$INSTDIR\config.toml"
    Delete "$INSTDIR\.db"

    ; Remove uninstaller itself
    Delete "$INSTDIR\uninstall.exe"
    RMDir "$INSTDIR"

    ; --- Remove registry entries ---
    ${If} $InstallMode == "system"
        DeleteRegKey HKLM "${UNINSTALL_KEY}"
    ${Else}
        DeleteRegKey HKCU "${UNINSTALL_KEY}"
    ${EndIf}
SectionEnd

; ============================================================================
; Uninstaller: Remove Windows Terminal profile
; ============================================================================
Function un.RemoveWindowsTerminalProfile
    Push $0
    Push $1

    StrCpy $0 "$LOCALAPPDATA\Packages\Microsoft.WindowsTerminal_8wekyb3d8bbwe\LocalState\settings.json"
    IfFileExists $0 _un_wt_found

    StrCpy $0 "$LOCALAPPDATA\Packages\Microsoft.WindowsTerminalPreview_8wekyb3d8bbwe\LocalState\settings.json"
    IfFileExists $0 _un_wt_found

    Goto _un_wt_done

    _un_wt_found:
    FileOpen $1 "$PLUGINSDIR\rm_wt_profile.ps1" w
    FileWrite $1 '$$settingsPath = "$0"$\r$\n'
    FileWrite $1 '$$json = Get-Content $$settingsPath -Raw | ConvertFrom-Json$\r$\n'
    FileWrite $1 '$$guid = "{e5a83867-4f30-4a6e-b0ea-2a4b9c0a6f73}"$\r$\n'
    FileWrite $1 '$$json.profiles.list = @($$json.profiles.list | Where-Object { $$_.guid -ne $$guid })$\r$\n'
    FileWrite $1 '$$json | ConvertTo-Json -Depth 100 | Set-Content $$settingsPath -Encoding UTF8$\r$\n'
    FileClose $1

    nsExec::ExecToLog 'powershell.exe -NoProfile -ExecutionPolicy Bypass -File "$PLUGINSDIR\rm_wt_profile.ps1"'
    Pop $0 ; consume nsExec return value

    _un_wt_done:
    Pop $1
    Pop $0
FunctionEnd

!endif ; INSTFILES_NSH
