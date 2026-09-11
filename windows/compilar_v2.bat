@echo off
title Compilar Buscaminas v2 (Rust)
cd /d "%~dp0.."

echo ========================================================
echo   Compilando Buscaminas v2 en Rust (Release)...
echo ========================================================

set PATH=%USERPROFILE%\.cargo\bin;%PATH%

where cargo >nul 2>nul
if %ERRORLEVEL% neq 0 (
    echo Error: Cargo/Rust no esta en el PATH.
    pause
    exit /b 1
)

cargo build --release
if %ERRORLEVEL% equ 0 (
    copy /y "target\release\buscaminas_v2_rust.exe" "windows\Buscaminas_v2.exe"
    echo.
    echo ========================================================
    echo   Compilacion exitosa!
    echo   Ejecutable: windows\Buscaminas_v2.exe
    echo ========================================================
) else (
    echo.
    echo Error durante la compilacion.
)

pause
