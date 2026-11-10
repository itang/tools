param()

$ErrorActionPreference = 'Stop'

$root = Split-Path -Parent $PSScriptRoot
$zip = Join-Path $root '.nsh\out\portradar.zip'
if (-not (Test-Path $zip)) {
  throw "未找到 $zip，请先运行 nsh package --target windows"
}
$meta = Get-Content (Join-Path $root 'nsh.json') -Raw -Encoding UTF8 | ConvertFrom-Json
$setupName = 'PortRadarSetup-{0}.exe' -f $meta.version

$stage = Join-Path $root '.nsh\installer\payload'
if (Test-Path $stage) {
  Remove-Item -Path $stage -Recurse -Force
}
New-Item -ItemType Directory -Path $stage -Force | Out-Null
Copy-Item -Path $zip -Destination $stage
$payloadFiles = @('portradar.zip')
foreach ($name in @('install.cmd', 'install.ps1', 'uninstall.ps1')) {
  Copy-Item -Path (Join-Path $PSScriptRoot $name) -Destination $stage
  $payloadFiles += $name
}

$dist = Join-Path $root 'dist'
New-Item -ItemType Directory -Path $dist -Force | Out-Null
$target = Join-Path $dist $setupName
if (Test-Path $target) {
  Remove-Item -Path $target -Force
}

$strings = for ($i = 0; $i -lt $payloadFiles.Count; $i++) {
  'FILE{0}="{1}"' -f $i, $payloadFiles[$i]
}
$sources = for ($i = 0; $i -lt $payloadFiles.Count; $i++) {
  '%FILE{0}%=' -f $i
}
$sed = @"
[Version]
Class=IEXPRESS
SEDVersion=3
[Options]
PackagePurpose=InstallApp
ShowInstallProgramWindow=0
HideExtractAnimation=0
UseLongFileName=1
InsideCompressed=0
CAB_FixedSize=0
CAB_ResvCodeSigning=0
RebootMode=N
InstallPrompt=%InstallPrompt%
DisplayLicense=%DisplayLicense%
FinishMessage=%FinishMessage%
TargetName=%TargetName%
FriendlyName=%FriendlyName%
AppLaunched=%AppLaunched%
PostInstallCmd=%PostInstallCmd%
AdminQuietInstCmd=%AdminQuietInstCmd%
UserQuietInstCmd=%UserQuietInstCmd%
SourceFiles=SourceFiles
[Strings]
InstallPrompt=
DisplayLicense=
FinishMessage=
TargetName=$target
FriendlyName=PortRadar Setup
AppLaunched=install.cmd
PostInstallCmd=<None>
AdminQuietInstCmd=install.cmd
UserQuietInstCmd=install.cmd
$($strings -join "`r`n")
[SourceFiles]
SourceFiles0=$stage\
[SourceFiles0]
$($sources -join "`r`n")
"@
$sedPath = Join-Path $root '.nsh\installer\setup.sed'
$sed = $sed.Replace("`r`n", "`n").Replace("`n", "`r`n")
[IO.File]::WriteAllText($sedPath, $sed, [Text.Encoding]::Default)

$iexpress = Join-Path $env:WINDIR 'System32\iexpress.exe'
$proc = Start-Process -FilePath $iexpress -ArgumentList @('/N', '/Q', $sedPath) -Wait -PassThru
if ($proc.ExitCode -ne 0 -or -not (Test-Path $target)) {
  throw "IExpress 打包失败（退出码 $($proc.ExitCode)）"
}
Write-Host "安装包已生成：$target"
