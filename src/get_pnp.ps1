$usbDrives = Get-WmiObject -class Win32_DiskDrive | Where-Object { $_.InterfaceType -eq "USB" }

$returnString = ""
$hasher = [System.Security.Cryptography.HashAlgorithm]::Create('sha256')
foreach ($drive in $usbDrives){
    $DeviceString = ($drive.Signature).ToString() + ($drive.Size).ToString() + $drive.SerialNumber + $drive.PNPDeviceID + $drive.Model + $drive.Caption
    $hash = $hasher.ComputeHash([System.Text.Encoding]::UTF8.GetBytes($DeviceString))
    $hashString = [System.BitConverter]::ToString($hash)
    $hashString = $hashString.Replace('-', '')
    
    $returnString = $returnString + $hashString + ";" + $drive.Model + ";"
}

return $returnString