; ============================================================================
; license.nsh — License agreement page
; ============================================================================

!ifndef LICENSE_NSH
!define LICENSE_NSH

; Use radio buttons so the user must explicitly accept
!define MUI_LICENSEPAGE_RADIOBUTTONS

; Insert the license page into the page sequence
!insertmacro MUI_PAGE_LICENSE "${MUSH_LICENSE_FILE}"

!endif ; LICENSE_NSH
