$ErrorActionPreference = 'Stop';
$toolsDir   = $(Split-Path -parent $MyInvocation.MyCommand.Definition)
$url64      = 'https://github.com/sachin-razz/rcurl/releases/latest/download/rcurl-windows-x86_64.zip'

$packageArgs = @{
  packageName   = 'rcurl'
  unzipLocation = $toolsDir
  url64Bit      = $url64
  checksum64    = ''
  checksumType64= 'sha256'
}

Install-ChocolateyZipPackage @packageArgs
