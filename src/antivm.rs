#[cfg(windows)]

use winapi::um::debugapi::IsDebuggerPresent; //To check if program is being debugged or not.
use winapi::um::sysinfoapi::GetTickCount; //To get the actual time scince the system is started.
use winapi::um::winuser::{GetLastInputInfo, LASTINPUTINFO, GetCursorPos}; //To get last user input details.
use winapi::um::synchapi::Sleep; //To make program sleep.
use winapi::um::winuser::{GetAsyncKeyState, VK_LBUTTON}; //To check key press event on the system.
use winapi::shared::windef::POINT; //Get currunt pointer position.

//Function to check ideal time of the system.
pub fn check_ideal_time() -> bool {
    unsafe{
        let mut last_user_input_info = LASTINPUTINFO{
            cbSize: std::mem::size_of::<LASTINPUTINFO>() as u32,
            dwTime: 0u32
        };

        GetLastInputInfo(&mut last_user_input_info);

        let count = (GetTickCount() - last_user_input_info.dwTime) / 1000;
        if count > 60{
            return true;
        }
        return false;

    }
}

//Function to check if debugger is present or not
pub fn check_debugger() -> bool{
    unsafe {
        match IsDebuggerPresent(){
            0 =>{
               println!("Debugger is not present. Continue...");
               return false;
            },

            _ => {
                println!("Debugger is present. Exiting execution!!");
                return true;
            }
        }
    }
}

//Function to make program sleep
pub fn make_sleep(){
    unsafe{
        Sleep(10000)
    }
}

//Function to check some number of right clicks on the machine
pub fn check_mouse_clicks(minimum_clicks: u32){
    unsafe{
        let mut count = 0;

        while count < minimum_clicks{
            let get_click_info = GetAsyncKeyState(VK_LBUTTON);

            if get_click_info >> 15 == -1{
                count += 1;
            }

            Sleep(100);
        }

        println!("Yea, 5 mouse clicks.");
    }
}

//Function to check the cursor position on the system.
pub fn validate_cursor_position() -> bool{
    unsafe{
        let mut current_cursor = POINT{x: 0i32, y:0i32};

        GetCursorPos(&mut current_cursor);

        let x = current_cursor.x;
        let y = current_cursor.y;

        Sleep(5000);

        GetCursorPos(&mut current_cursor);

        if x == current_cursor.x && y == current_cursor.y{
            return false;
        }
    }
    return true;
}