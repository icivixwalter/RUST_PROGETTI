ECHO OFF

:------------------------------- ATTIVA PRIMA I SALVATAGGI NELLA CARTELLA CORRENTE E COPIA NELLA CARTELLA GIT
ECHO.
ECHO ATTIVA I SALVATAGGI NELLA CARTELLA CORRENTE E COPIA IN GIT
ECHO.

CALL Rust_Progetti_Archiviazione.bat

START "ATTIVA GIT" CALL "c:\CASA\PROGRAMMI\GIT_DESKTOP\CASA\PROGRAMMI\RUST_PROGETTI\GIT_FILE_(PULL_RUST_PROGETTI).bat"
