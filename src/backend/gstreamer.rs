
use gstreamer::{self as gstlib};
use gstreamer::prelude::*;
use glib::ControlFlow;

use crate::backend::Backend;
use std::io::{self, Read};


impl Backend for gstlib::Element{
    fn pause(&mut self){
        self.set_state(gstlib::State::Paused).unwrap();
    }
    fn play(&mut self){
        self.set_state(gstlib::State::Playing).unwrap();
    }
    fn load(&mut self,url: &str){
        self.set_property("uri", url);
    }
    fn end(&mut self){
        self.set_state(gstlib::State::Null).unwrap();
        // self.set_property("stop", true);
    }
    
    fn events(&mut self) {
        let bus = self.bus().unwrap();
        let _ = bus.add_watch(|_, msg| {
            match msg.view() {
                
                gstlib::MessageView::Error(err) => {
                    println!("Error: {:?}", err.error());
                }
                gstlib::MessageView::Eos(_) => {
                    println!("End of stream");
                }
                gstlib::MessageView::StateChanged(state) => {
                    println!("State changed: {:?}", state.current());
                }
                gstlib::MessageView::StreamStart(_) => {
                    println!("Stream started");
                }
                gstlib::MessageView::Buffering(progress) => {
                    println!("Buffering: {}%", progress.percent());
                }
                _ => {println!("Received: {:?}",msg);} // Handle other message types as needed
            }
            ControlFlow::Continue
        }).expect("Failed to add bus watch");
        loop {
            // Process pending messages
            bus.timed_pop(gstlib::ClockTime::from_mseconds(100));
    
            // Check for user input to quit
            if let Some('q') = get_char_input() {
                println!("Stopping playback...");
                self.end();
                break; // Exit the loop
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


pub fn gst_play(url: &str){
    let mut gst = gst_init();
    gst.load(url);
    gst.play();
    gst.events();
}


fn gst_init() -> gstlib::Element{
    gstlib::init().unwrap();
    // Create the pipeline using playbin
    gstlib::ElementFactory::make("playbin").build().unwrap()
}