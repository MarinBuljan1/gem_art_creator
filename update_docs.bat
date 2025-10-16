@echo off
setlocal

echo === Updating docs folder (Wasm + CSS + assets) ===

REM Ensure target directories exist
if not exist docs mkdir docs
if not exist docs\pkg mkdir docs\pkg
if not exist docs\static mkdir docs\static
if not exist docs\icons mkdir docs\icons

REM Build WebAssembly to docs/pkg
echo -- Building Wasm to docs\pkg
wasm-pack build --target web --out-dir docs/pkg --out-name yew_project
if errorlevel 1 goto :error

REM Build CSS to docs/static
echo -- Building CSS to docs\static
sass static/style.scss:docs/static/style.css
if errorlevel 1 goto :error

REM Copy HTML/manifest and optional assets
echo -- Copying HTML/manifest and assets
copy /Y index.html docs\index.html >nul
copy /Y manifest.json docs\manifest.json >nul
if exist static\style.css.map copy /Y static\style.css.map docs\static\style.css.map >nul
if exist static\DejaVuSans.ttf copy /Y static\DejaVuSans.ttf docs\static\DejaVuSans.ttf >nul

REM Copy icons
xcopy /E /I /Y icons docs\icons >nul

echo === Done. Docs updated in docs\ ===
exit /b 0

:error
echo.
echo ERROR: update_docs failed. Ensure "wasm-pack" and "sass" are installed and on PATH.
exit /b 1

