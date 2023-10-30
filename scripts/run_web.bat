@echo off
cd %~dp0../comfy_breakout/dist
start http://localhost:8000
python3 -m http.server 8000
