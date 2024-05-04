rule Detect_File_Drop_encrypt_date_txt {
    meta:
        description = "Detects the file 'encrypt_date.txt' dropped on the system"
    strings:
        $file_name = "encrypt_date.txt"
    condition:
        $file_name
}
