@echo off
echo =========================================
echo Building rtop (Rust System Monitor)
echo =========================================
cargo build --release
if %ERRORLEVEL% NEQ 0 (
    echo [ERROR] Cargo build failed!
    exit /b %ERRORLEVEL%
)
copy /y target\release\rtop.exe .\rtop.exe
echo =========================================
echo Build successful! Run .\rtop.exe to start.
echo =========================================
