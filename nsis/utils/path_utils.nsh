; ============================================================================
; path_utils.nsh — Shared PATH manipulation macros for Mush installer
; ============================================================================

!ifndef PATH_UTILS_NSH
!define PATH_UTILS_NSH

!include "WinMessages.nsh"

; ============================================================================
; Installer functions
; ============================================================================

; ----------------------------------------------------------------------------
; AddToPath — Appends a directory to the system or user PATH
;   Expects: $R8 = directory, $R9 = "system" or "user"
; ----------------------------------------------------------------------------
Function AddToPath
    Push $0
    Push $1
    Push $2

    StrCmp $R9 "system" 0 _atp_user
        ReadRegStr $0 HKLM "SYSTEM\CurrentControlSet\Control\Session Manager\Environment" "Path"
        Goto _atp_check
    _atp_user:
    ReadRegStr $0 HKCU "Environment" "Path"

    _atp_check:
    ; Check if directory is already in PATH
    Push "$0"
    Push "$R8"
    Call IsInPath
    Pop $1
    StrCmp $1 "1" _atp_done

    ; Append with semicolon separator
    StrLen $2 $0
    IntCmp $2 0 _atp_empty
    StrCpy $0 "$0;$R8"
    Goto _atp_write

    _atp_empty:
    StrCpy $0 "$R8"

    _atp_write:
    StrCmp $R9 "system" 0 _atp_write_user
        WriteRegExpandStr HKLM "SYSTEM\CurrentControlSet\Control\Session Manager\Environment" "Path" "$0"
        Goto _atp_broadcast
    _atp_write_user:
    WriteRegExpandStr HKCU "Environment" "Path" "$0"

    _atp_broadcast:
    SendMessage ${HWND_BROADCAST} ${WM_SETTINGCHANGE} 0 "STR:Environment" /TIMEOUT=5000

    _atp_done:
    Pop $2
    Pop $1
    Pop $0
FunctionEnd

; ----------------------------------------------------------------------------
; IsInPath — Checks if a directory is already in a PATH string
;   Stack: PATH_STRING, DIR_TO_FIND -> returns "1" or "0"
; ----------------------------------------------------------------------------
Function IsInPath
    Exch $0 ; DIR_TO_FIND
    Exch
    Exch $1 ; PATH_STRING
    Push $2
    Push $3
    Push $4

    StrCpy $2 $1

    _iip_loop:
    StrLen $3 $2
    IntCmp $3 0 _iip_notfound

    Push $2
    Push ";"
    Call SplitFirstStrPart
    Pop $3
    Pop $2

    StrCmp $3 $0 _iip_found
    Goto _iip_loop

    _iip_found:
    StrCpy $0 "1"
    Goto _iip_done

    _iip_notfound:
    StrCpy $0 "0"

    _iip_done:
    Pop $4
    Pop $3
    Pop $2
    Pop $1
    Exch $0
FunctionEnd

; ----------------------------------------------------------------------------
; SplitFirstStrPart — Splits a string at the first occurrence of a delimiter
;   Stack: STRING, DELIMITER -> FIRST_PART, REMAINDER
; ----------------------------------------------------------------------------
Function SplitFirstStrPart
    Exch $0 ; delimiter
    Exch
    Exch $1 ; string
    Push $2
    Push $3
    Push $4

    StrLen $2 $0 ; delimiter length
    StrLen $3 $1 ; string length
    StrCpy $4 0  ; position

    _sfsp_loop:
    IntCmp $4 $3 _sfsp_notfound
    StrCpy $R0 $1 $2 $4
    StrCmp $R0 $0 _sfsp_found
    IntOp $4 $4 + 1
    Goto _sfsp_loop

    _sfsp_found:
    StrCpy $R0 $1 $4       ; first part
    IntOp $4 $4 + $2
    StrCpy $R1 $1 "" $4    ; remainder
    StrCpy $0 $R1
    StrCpy $1 $R0
    Goto _sfsp_done

    _sfsp_notfound:
    StrCpy $0 ""  ; no remainder
    ; $1 stays as the whole string (first part)

    _sfsp_done:
    Pop $4
    Pop $3
    Pop $2
    Exch $0 ; remainder below
    Exch
    Exch $1 ; first part on top
FunctionEnd

; ============================================================================
; Uninstaller functions
; ============================================================================

; ----------------------------------------------------------------------------
; un.RemoveFromPath — Removes a directory from the system or user PATH
;   Expects: $R8 = directory, $R9 = "system" or "user"
; ----------------------------------------------------------------------------
Function un.RemoveFromPath
    Push $0
    Push $1
    Push $2
    Push $3
    Push $4

    StrCmp $R9 "system" 0 _un_rfp_user
        ReadRegStr $0 HKLM "SYSTEM\CurrentControlSet\Control\Session Manager\Environment" "Path"
        Goto _un_rfp_process
    _un_rfp_user:
    ReadRegStr $0 HKCU "Environment" "Path"

    _un_rfp_process:
    StrCpy $1 "" ; new path accumulator
    StrCpy $2 $0 ; remaining

    _un_rfp_loop:
    StrLen $3 $2
    IntCmp $3 0 _un_rfp_write

    Push $2
    Push ";"
    Call un.SplitFirstStrPart
    Pop $3 ; first part
    Pop $2 ; remainder

    ; Compare with target — skip if it matches
    StrCmp $3 $R8 _un_rfp_loop

    ; Keep this entry
    StrLen $4 $1
    IntCmp $4 0 _un_rfp_first
        StrCpy $1 "$1;$3"
        Goto _un_rfp_loop
    _un_rfp_first:
    StrCpy $1 $3
    Goto _un_rfp_loop

    _un_rfp_write:
    StrCmp $R9 "system" 0 _un_rfp_write_user
        WriteRegExpandStr HKLM "SYSTEM\CurrentControlSet\Control\Session Manager\Environment" "Path" "$1"
        Goto _un_rfp_broadcast
    _un_rfp_write_user:
    WriteRegExpandStr HKCU "Environment" "Path" "$1"

    _un_rfp_broadcast:
    SendMessage ${HWND_BROADCAST} ${WM_SETTINGCHANGE} 0 "STR:Environment" /TIMEOUT=5000

    Pop $4
    Pop $3
    Pop $2
    Pop $1
    Pop $0
FunctionEnd

Function un.SplitFirstStrPart
    Exch $0
    Exch
    Exch $1
    Push $2
    Push $3
    Push $4

    StrLen $2 $0
    StrLen $3 $1
    StrCpy $4 0

    _un_sfsp_loop:
    IntCmp $4 $3 _un_sfsp_notfound
    StrCpy $R0 $1 $2 $4
    StrCmp $R0 $0 _un_sfsp_found
    IntOp $4 $4 + 1
    Goto _un_sfsp_loop

    _un_sfsp_found:
    StrCpy $R0 $1 $4
    IntOp $4 $4 + $2
    StrCpy $R1 $1 "" $4
    StrCpy $0 $R1
    StrCpy $1 $R0
    Goto _un_sfsp_done

    _un_sfsp_notfound:
    StrCpy $0 ""

    _un_sfsp_done:
    Pop $4
    Pop $3
    Pop $2
    Exch $0 ; remainder below
    Exch
    Exch $1 ; first part on top
FunctionEnd

!endif ; PATH_UTILS_NSH
