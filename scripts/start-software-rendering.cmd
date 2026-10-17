@echo off
setlocal
set "ZAPFAST_SOFTWARE_RENDERING=1"
start "" /D "%~dp0" "%~dp0zapfast.exe" %*
endlocal
