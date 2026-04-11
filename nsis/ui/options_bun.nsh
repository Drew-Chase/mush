; ============================================================================
; options_bun.nsh — Bun runtime installation option page
; ============================================================================

!ifndef OPTIONS_BUN_NSH
!define OPTIONS_BUN_NSH

!include "LogicLib.nsh"

Var BunDialog
Var ChkInstallBun
Var OptInstallBun

; Register the custom page
Page custom BunOptionsPage BunOptionsPageLeave

; ----------------------------------------------------------------------------
; Page creation
; ----------------------------------------------------------------------------
Function BunOptionsPage
    !insertmacro MUI_HEADER_TEXT "Bun Runtime" "Bun is required for TypeScript script support."

    nsDialogs::Create 1018
    Pop $BunDialog
    ${If} $BunDialog == error
        Abort
    ${EndIf}

    ${NSD_CreateLabel} 0 0 100% 24u "Mush can run TypeScript projects as shell commands. This requires the Bun runtime to be installed."
    Pop $0

    ${NSD_CreateCheckbox} 20u 32u 280u 16u "Install &Bun runtime"
    Pop $ChkInstallBun
    ${NSD_SetState} $ChkInstallBun ${BST_UNCHECKED}

    ${NSD_CreateLabel} 40u 50u 260u 12u "Downloads and installs Bun from bun.sh. Skip if already installed."
    Pop $0
    SetCtlColors $0 "888888" transparent

    nsDialogs::Show
FunctionEnd

; ----------------------------------------------------------------------------
; Page leave — read checkbox state
; ----------------------------------------------------------------------------
Function BunOptionsPageLeave
    ${NSD_GetState} $ChkInstallBun $OptInstallBun
FunctionEnd

!endif ; OPTIONS_BUN_NSH
