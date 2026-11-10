@echo off
chcp 65001 >nul
title PortRadar Setup
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%~dp0install.ps1" %*
if errorlevel 1 (
  echo.
  echo Install failed. Press any key to close.
  pause >nul
)
