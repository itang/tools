param(
  [string]$InstallDir = (Join-Path $env:LOCALAPPDATA 'Programs\PortRadar')
)

$ErrorActionPreference = 'Stop'

$displayName = '端口雷达'
$publisher = 'com.example.portradar'
$zip = Join-Path $PSScriptRoot 'portradar.zip'
if (-not (Test-Path $zip)) {
  throw "安装包不完整：缺少 $zip"
}

Get-Process -Name portradar -ErrorAction SilentlyContinue | Stop-Process -Force
Start-Sleep -Milliseconds 300

$staging = Join-Path $env:TEMP ('portradar-' + [guid]::NewGuid().ToString('n'))
Expand-Archive -Path $zip -DestinationPath $staging -Force
$source = Join-Path $staging 'portradar'
$meta = Get-Content (Join-Path $source 'nsh\app.json') -Raw -Encoding UTF8 | ConvertFrom-Json
$version = $meta.version

New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
Copy-Item -Path (Join-Path $source '*') -Destination $InstallDir -Recurse -Force
Copy-Item -Path (Join-Path $PSScriptRoot 'uninstall.ps1') -Destination $InstallDir -Force
Remove-Item -Path $staging -Recurse -Force

$exe = Join-Path $InstallDir 'portradar.exe'
$uninstaller = Join-Path $InstallDir 'uninstall.ps1'
$key = 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Uninstall\PortRadar'
New-Item -Path $key -Force | Out-Null
$size = [math]::Round(((Get-ChildItem $InstallDir -Recurse -File | Measure-Object Length -Sum).Sum) / 1KB)
$uninstallCommand = "powershell.exe -NoProfile -ExecutionPolicy Bypass -WindowStyle Hidden -File `"$uninstaller`""
$props = @(
  @{ Name = 'DisplayName'; Value = $displayName; Type = 'String' },
  @{ Name = 'DisplayVersion'; Value = $version; Type = 'String' },
  @{ Name = 'Publisher'; Value = $publisher; Type = 'String' },
  @{ Name = 'InstallLocation'; Value = $InstallDir; Type = 'String' },
  @{ Name = 'DisplayIcon'; Value = "$exe,0"; Type = 'String' },
  @{ Name = 'UninstallString'; Value = $uninstallCommand; Type = 'String' },
  @{ Name = 'QuietUninstallString'; Value = $uninstallCommand; Type = 'String' },
  @{ Name = 'NoModify'; Value = 1; Type = 'DWord' },
  @{ Name = 'NoRepair'; Value = 1; Type = 'DWord' },
  @{ Name = 'EstimatedSize'; Value = $size; Type = 'DWord' }
)
foreach ($prop in $props) {
  New-ItemProperty -Path $key -Name $prop.Name -Value $prop.Value -PropertyType $prop.Type -Force | Out-Null
}

$shell = New-Object -ComObject WScript.Shell
$menuDir = Join-Path ([Environment]::GetFolderPath('Programs')) $displayName
New-Item -ItemType Directory -Path $menuDir -Force | Out-Null
foreach ($dir in @($menuDir, [Environment]::GetFolderPath('Desktop'))) {
  $shortcut = $shell.CreateShortcut((Join-Path $dir ($displayName + '.lnk')))
  $shortcut.TargetPath = $exe
  $shortcut.WorkingDirectory = $InstallDir
  $shortcut.IconLocation = "$exe,0"
  $shortcut.Description = $displayName
  $shortcut.Save()
}
$uninstallShortcut = $shell.CreateShortcut((Join-Path $menuDir ($displayName + ' 卸载.lnk')))
$uninstallShortcut.TargetPath = 'powershell.exe'
$uninstallShortcut.Arguments = "-NoProfile -ExecutionPolicy Bypass -File `"$uninstaller`""
$uninstallShortcut.WorkingDirectory = $InstallDir
$uninstallShortcut.IconLocation = "$exe,0"
$uninstallShortcut.Description = "卸载 $displayName"
$uninstallShortcut.Save()

Write-Host "已安装到 $InstallDir"
Start-Process -FilePath (Join-Path $env:WINDIR 'explorer.exe') -ArgumentList "`"$exe`""
