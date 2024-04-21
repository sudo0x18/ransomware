#[cfg(windows)]

mod antivm; //AntiVM Module

use crate::antivm::{check_debugger, check_ideal_time, make_sleep, check_mouse_clicks, validate_cursor_position};

fn main() {

    //make_sleep();
    
    if check_debugger(){
        std::process::exit(0);
    }

    if check_ideal_time(){
        println!("System is Ideal..");
        std::process::exit(0);
    }

    //check_mouse_clicks(5);
    if validate_cursor_position(){
        println!("Hello World!!");
        loop{}
    }
}
