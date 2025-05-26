@echo off
setlocal

:: Cek apakah file .env ada
if not exist ".env" (
    echo File .env tidak ditemukan di direktori saat ini.
    exit /b 1
)

echo Memuat variabel dari .env...

:: Baca file .env baris per baris
for /f "tokens=*" %%a in ('type ".env" ^| findstr /v "^$" ^| findstr /v "^#"') do (
    echo.%%a | findstr "=" >nul && (
        for /f "tokens=1,2 delims==" %%b in ("%%a") do (
            set "key=%%b"
            set "value=%%c"
            :: Trim whitespace
            call :trimValue key value
            echo Set variabel: %%b=%%c
            setx %%b %%c >nul
        )
    )
)

echo.
echo Semua variabel telah dimuat.
endlocal
goto :eof

:: Fungsi trim (sederhana)
:trimValue
set "%1=%[%1]%"
set "%2=%[%2]%"
goto :eof