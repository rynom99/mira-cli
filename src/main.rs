use std::env;
use mira_cli::mira;
use mira_cli::mira::ScreenMode;

fn main() {
    let mira_device = mira::new();    
    let args: Vec<String> = env::args().collect();
    let mut index = 0;
    let len = args.len();
    let mira = &mira_device;
    while len != index  {
        let argument = &args[index].to_lowercase();
        match argument.trim(){
            "--mode" => {
                index += 1;
                let mode = &args[index].to_lowercase();
                match mode.trim(){
                    "speed" => {
                        mira.set_screen_mode(ScreenMode::Speed);
                    }
                    "text" => {
                        mira.set_screen_mode(ScreenMode::Text);
                    }
                    "image" => {
                        mira.set_screen_mode(ScreenMode::Image);
                    }
                    "video" => {
                        mira.set_screen_mode(ScreenMode::Video);
                    }
                    "read" => {
                        mira.set_screen_mode(ScreenMode::Read);
                    }
                    _ => {}
                }
            }
            "--refresh" => {
                index += 1;
                let mode = &args[index].to_lowercase();
                match mode.trim(){
                    "1"|"true" => {mira.refresh();}
                    _ => {}
                }
            }
            _ => {}
        };
        index += 1;
    }
}