; ============================================================================
; install_mode.nsh — User vs System install selection (custom page)
; ============================================================================

!ifndef INSTALL_MODE_NSH
!define INSTALL_MODE_NSH

!include "LogicLib.nsh"

Var InstallMode       ; "user" or "system"
Var InstModeDialog
Var InstModeRadioUser
Var InstModeRadioSystem
Var IsAdmin

; Register the custom page
Page custom InstallModePage InstallModePageLeave

; ----------------------------------------------------------------------------
; Page creation
; ----------------------------------------------------------------------------
Function InstallModePage
    ; Check admin status
    UserInfo::GetAccountType
    Pop $0
    ${If} $0 == "Admin"
        StrCpy $IsAdmin "1"
    ${Else}
        StrCpy $IsAdmin "0"
    ${EndIf}

    !insertmacro MUI_HEADER_TEXT "Installation Scope" "Choose whether to install for the current user or all users."

    nsDialogs::Create 1018
    Pop $InstModeDialog
    ${If} $InstModeDialog == error
        Abort
    ${EndIf}

    ${NSD_CreateLabel} 0 0 100% 24u "Select the installation scope. Installing for all users requires administrator privileges and installs to Program Files. Installing for the current user only installs to your local application data folder."
    Pop $0

    ; Current user radio — default
    ${NSD_CreateRadioButton} 20u 40u 280u 16u "Install for &current user only ($LOCALAPPDATA\${APP_NAME})"
    Pop $InstModeRadioUser
    ${NSD_SetState} $InstModeRadioUser ${BST_CHECKED}

    ; All users radio
    ${NSD_CreateRadioButton} 20u 60u 280u 16u "Install for &all users — requires administrator ($PROGRAMFILES\${APP_NAME})"
    Pop $InstModeRadioSystem

    ; If not admin, disable the system-wide option
    ${If} $IsAdmin == "0"
        EnableWindow $InstModeRadioSystem 0
        ${NSD_CreateLabel} 40u 78u 260u 16u "(Run installer as Administrator to enable this option)"
        Pop $0
        SetCtlColors $0 "FF0000" transparent
    ${EndIf}

    nsDialogs::Show
FunctionEnd

; ----------------------------------------------------------------------------
; Page leave — apply selection
; ----------------------------------------------------------------------------
Function InstallModePageLeave
    ${NSD_GetState} $InstModeRadioSystem $0
    ${If} $0 == ${BST_CHECKED}
        StrCpy $InstallMode "system"
        SetShellVarContext all
        StrCpy $INSTDIR "$PROGRAMFILES\${APP_NAME}"
    ${Else}
        StrCpy $InstallMode "user"
        SetShellVarContext current
        StrCpy $INSTDIR "$LOCALAPPDATA\${APP_NAME}"
    ${EndIf}
FunctionEnd

!endif ; INSTALL_MODE_NSH
