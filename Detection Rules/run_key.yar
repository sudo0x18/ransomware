rule Detect_Run_Key_Rusty_Ransomware {
    meta:
        description = "Detects the presence of Registry Key Rusty Ransomware and its associated value in the Run key"
    strings:
        $key_string = "HKEY_LOCAL_MACHINE\\Software\\Microsoft\\Windows\\CurrentVersion\\Run"
        $value_string = "Rusty Ransomware"
    condition:
        $key_string and $value_string
}