rule Detect_Ransomware_ransomnote_exe {
    meta:
        description = "Detects the ransomware executable 'ransomnote.exe'"
    strings:
        $file_name = "ransomnote.exe"
    condition:
        $file_name
}
