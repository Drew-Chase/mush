; ============================================================================
; directory.nsh — Directory selection page
; ============================================================================

!ifndef DIRECTORY_NSH
!define DIRECTORY_NSH

; The directory page lets the user override the default set by install_mode.
; $INSTDIR is already set by InstallModePageLeave before this page appears.
!insertmacro MUI_PAGE_DIRECTORY

!endif ; DIRECTORY_NSH
