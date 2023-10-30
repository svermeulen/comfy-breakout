@echo off
call %~dp0build_web.bat
if errorlevel 1 exit /b 1
call %~dp0run_web.bat
