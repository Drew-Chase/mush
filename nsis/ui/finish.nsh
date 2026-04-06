; ============================================================================
; finish.nsh — Installation complete page
; ============================================================================

!ifndef FINISH_NSH
!define FINISH_NSH

; Offer to launch Mush after install
!define MUI_FINISHPAGE_RUN "$INSTDIR\bin\mush.exe"
!define MUI_FINISHPAGE_RUN_TEXT "Launch Mush"

; Show a link to the project page
!define MUI_FINISHPAGE_LINK "Visit the Mush project on GitHub"
!define MUI_FINISHPAGE_LINK_LOCATION "${APP_URL}"

!insertmacro MUI_PAGE_FINISH

!endif ; FINISH_NSH
