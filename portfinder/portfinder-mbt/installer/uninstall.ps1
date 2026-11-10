$ErrorActionPreference = 'SilentlyContinue'

$displayName = '端口雷达'
$dir = $PSScriptRoot

Get-Process -Name portradar -ErrorAction SilentlyContinue | Stop-Process -Force
Start-Sleep -Milliseconds 500

Remove-Item -Path 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\PortRadar' -Recurse -Force
Remove-Item -Path (Join-Path ([Environment]::GetFolderPath('Programs')) $displayName) -Recurse -Force
Remove-Item -Path (Join-Path ([Environment]::GetFolderPath('Desktop')) ($displayName + '.lnk')) -Force

Set-Location $env:TEMP
Remove-Item -LiteralPath $dir -Recurse -Force
