# Simple USB fingerprint for studies
Project is an assignment for laboratory class.
## Project realise following functionalities
1. detect mounted USBs
2. determine fingerprint for USBs
3. maintain black list and white list of fingerprints.


## user actions in context of lsts
1. user should be able to check whether new USBs are mounted or not. if new USBs are detected, program put them in  lists specified by user (blacklist or whitelist).
2. user should be able to move fingerprint from one list to another.
3. user should be able to check actual lists.

## implementation description
1. fingerprint is hash (SHA256) of concatenated following atributes: 
    * Signature
    * Size
    * SerialNumber
    * PNPDeviceID
    * Model
    * Caption
for more details check the microsoft docs: https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-diskdrive

2. Program use powershell script to determine fingerprint. (watch out! path to the script is hardcoded)
3. Maintained lists are stored in txt files. (watch out! paths to lits are hardcoded)
4. Program sees USB and attached to it fingerprint as structure called "Usb". it has atributes name and hash. Data stored in lists are actually instances of "Usb" structer in JSON format.
5. If program determine new fingerprints, it will write it at the end of program execution.