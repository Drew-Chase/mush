; ============================================================================
; options.nsh — Installation options page (PATH, Windows Terminal, etc.)
; ============================================================================

!ifndef OPTIONS_NSH
!define OPTIONS_NSH

!include "LogicLib.nsh"

Var OptionsDialog
Var ChkAddToPath
Var ChkWindowsTerminal
Var ChkStartMenu
Var ChkSetDefaultShell

Var OptAddToPath
Var OptWindowsTerminal
Var OptStartMenu
Var OptSetDefaultShell

; Register the custom page
Page custom OptionsPage OptionsPageLeave

; ----------------------------------------------------------------------------
; Page creation
; ----------------------------------------------------------------------------
Function OptionsPage
    !insertmacro MUI_HEADER_TEXT "Installation Options" "Configure additional options for Mush."

    nsDialogs::Create 1018
    Pop $OptionsDialog
    ${If} $OptionsDialog == error
        Abort
    ${EndIf}

    ${NSD_CreateLabel} 0 0 100% 16u "Select which additional features to configure:"
    Pop $0

    ; --- Add to PATH ---
    ${NSD_CreateCheckbox} 20u 22u 280u 16u "&Add Mush to PATH (recommended)"
    Pop $ChkAddToPath
    ${NSD_SetState} $ChkAddToPath ${BST_CHECKED}

    ${If} $InstallMode == "system"
        ${NSD_CreateLabel} 40u 38u 260u 12u "Mush will be added to the system PATH for all users."
    ${Else}
        ${NSD_CreateLabel} 40u 38u 260u 12u "Mush will be added to your user PATH."
    ${EndIf}
    Pop $0

    ; --- Windows Terminal profile ---
    ${NSD_CreateCheckbox} 20u 54u 280u 16u "Add Mush profile to &Windows Terminal"
    Pop $ChkWindowsTerminal
    ${NSD_SetState} $ChkWindowsTerminal ${BST_CHECKED}

    ; --- Start Menu shortcut ---
    ${NSD_CreateCheckbox} 20u 70u 280u 16u "Create &Start Menu shortcut"
    Pop $ChkStartMenu
    ${NSD_SetState} $ChkStartMenu ${BST_CHECKED}

    ; --- Set as default shell ---
    ${NSD_CreateCheckbox} 20u 86u 280u 16u "Set Mush as the &default shell"
    Pop $ChkSetDefaultShell
    ; Default: unchecked — this is aggressive for a new install
    ${NSD_SetState} $ChkSetDefaultShell ${BST_UNCHECKED}

    ${NSD_CreateLabel} 40u 102u 260u 12u "Sets COMSPEC to mush.exe. Only recommended if you are replacing your current shell."
    Pop $0
    SetCtlColors $0 "888888" transparent

    nsDialogs::Show
FunctionEnd

; ----------------------------------------------------------------------------
; Page leave — read checkbox states
; ----------------------------------------------------------------------------
Function OptionsPageLeave
    ${NSD_GetState} $ChkAddToPath $OptAddToPath
    ${NSD_GetState} $ChkWindowsTerminal $OptWindowsTerminal
    ${NSD_GetState} $ChkStartMenu $OptStartMenu
    ${NSD_GetState} $ChkSetDefaultShell $OptSetDefaultShell
FunctionEnd

!endif ; OPTIONS_NSH
