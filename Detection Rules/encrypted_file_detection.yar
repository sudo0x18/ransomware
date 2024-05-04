rule Detect_File_Extension_jay {
    meta:
        description = "Detects files with the extension '.jay'"
    strings:
        $file_ext = ".jay"
    condition:
        $file_ext
}
