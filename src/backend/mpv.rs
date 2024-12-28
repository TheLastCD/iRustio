
use mpv::MpvHandler;
use crate::backend::Backend;
use std::io::{self, Read};


impl Backend for MpvHandler{
    fn pause(&mut self){
        self.set_property("pause", true).expect("Error pausing file");
    }
    fn play(&mut self){
        self.set_property("pause", false).expect("Error playing file");   
    }
    fn load(&mut self,url: &str){
        self.command(&["loadfile", url]).expect("Error loading file");
    }
    fn end(&mut self){
        self.command(&["stop"]).expect("Error stopping file");
    }
    
    fn events(&mut self) {
        'main: loop {
            while let Some(event) = self.wait_event(1.0) {

                // println!("RECEIVED EVENT : {:?}", event);
                match event {  
                    //TODO provide correct logging
                    mpv::Event::Shutdown | mpv::Event::Idle => {
                        break 'main;
                    }
                    _ => {}
                };

            }

            println!("use q to quit & P to play/pause");
            let button = get_char_input().unwrap().to_string();
            match button.trim(){
                "q" =>{
                    println!("Stopping playback...");
                    self.end();
                    // break 'main;

                }
                "p" =>{
                    if self.get_property("pause").unwrap(){
                        println!("Resuming Playback...");
                        self.play();
                    }
                    else{
                        println!("Pausing Playback...");
                        self.pause();
                    }

                }
                _ =>{
                    
                }

            }

        }


    }
    
}


fn get_char_input() -> Option<char> {
    let mut buffer = [0; 1];
    match io::stdin().read(&mut buffer) {
        Ok(1) => Some(buffer[0] as char),
        _ => None,
    }
}


pub fn mpv_play(url: &str){
    let mut mpv = mpv_init();
    mpv.load(url);
    mpv.play();
    mpv.events();
}


fn mpv_init() -> MpvHandler{
    let mpv_builder = mpv::MpvHandlerBuilder::new().expect("Failed to init MPV builder");
    // mpv_builder.set_option("no-video",true).unwrap();
    mpv_builder.build().expect("Failed to build MPV handler")
}