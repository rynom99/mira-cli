pub mod mira {
    use core::time;
    use std::cmp;
    use std::thread;
    use hidapi::HidDevice;
    
    static USB_ID: u8 = 0x00;
    static VID: u16 = 0x0416;
    static PID: u16 = 0x5020;

    pub struct MiraDevice {
        device:HidDevice,
    }

    pub fn new() -> MiraDevice {
        let api = hidapi::HidApi::new().unwrap();
        let device = api.open(VID, PID).unwrap();
        MiraDevice {
            device
        }
    }

    impl MiraDevice{
        pub fn set_screen_mode(&self, mode:ScreenMode){
            match mode {
                ScreenMode::Speed => {
                    let _ = &self.set_screen(RefreshMode::A2, 8, 7, 0, 0, 0);
                }
                ScreenMode::Text => {
                    let _ = &self.set_screen(RefreshMode::A2, 7, 6, 1, 0, 0);
                }
                ScreenMode::Image => {
                    let _ = &self.set_screen(RefreshMode::DirectUpdate, 7, 5, 0, 0, 0);
                }
                ScreenMode::Video => {
                    let _ = &self.set_screen(RefreshMode::A2, 7, 6, 2, 10, 0);
                }
                ScreenMode::Read => {
                    let _ = &self.set_screen(RefreshMode::DirectUpdate,  7, 5, 3, 12, 10);
                }
            }
        }

        fn set_screen(&self, refresh_mode:RefreshMode, contrast: u8, speed: u8, dither_mode: u8, white_filter: u8, black_filter: u8) -> () {
            let _ = &self.set_refresh_mode(refresh_mode);
            let _ = &self.set_contrast(contrast);
            let _ = &self.set_speed(speed);
            let _ = &self.set_dither_mode(dither_mode);
            let _ = &self.set_color_filter(white_filter, black_filter);
            let _ = &self.refresh();
        }
        
        fn send(&self, buffer:&[u8]) -> () {
            let _ = &self.device.write(buffer).unwrap();
            thread::sleep(time::Duration::from_millis(500));
        }
        
        pub fn refresh(&self){
            let _ = &self.send(&[USB_ID, TypeMode::Refresh as u8]);
        }
        
        fn set_refresh_mode(&self, mode: RefreshMode){
            let _ = &self.send(&[USB_ID, TypeMode::RefreshMode as u8, mode as u8]);
        }
        
        fn set_speed(&self, speed: u8){
            let adjusted_speed = 11 - clamp(speed, 1, 7);
            let _ = &self.send(&[USB_ID, TypeMode::Speed as u8, adjusted_speed]);
        }
        
        fn set_contrast(&self, contrast: u8){
            let adjusted_contrast = clamp(contrast, 0, 15);
            let _ = &self.send(&[USB_ID, TypeMode::Contrast as u8, adjusted_contrast]);
        }
        
        fn set_dither_mode(&self, mode: u8){
            let adjusted_mode = clamp(mode, 0, 3);
            let _ = &self.send(&[USB_ID, TypeMode::DitherMode as u8, adjusted_mode]);
        }
        
        fn set_color_filter(&self, white_filter: u8, black_filter: u8){
            let adjusted_white = 255 - clamp(white_filter, 0, 254);
            let adjusted_black = clamp(black_filter, 0, 254);
            let _ = &self.send(&[USB_ID, TypeMode::ColorFilter as u8, adjusted_white, adjusted_black]);
        }
    }

    fn clamp(value:u8, min:u8, max:u8) -> u8 {
        cmp::min(cmp::max(min, value), max)
    }

    pub enum ScreenMode{
        Speed,
        Text,
        Image,
        Video,
        Read,
    }
    
    #[repr(u8)]
    enum RefreshMode {
        DirectUpdate = 0x01,
        // GrayUpdate = 0x02, 
        A2 = 0x03
    }

    #[repr(u8)]
    enum TypeMode {
        Refresh = 0x01,
        RefreshMode = 0x02,
        Speed = 0x04,
        Contrast = 0x05,
        // ColdLight = 0x06,
        // WarmLight = 0x07,
        DitherMode = 0x09,
        ColorFilter = 0x11
    }

}