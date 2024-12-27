pub mod mpv;

// use std::error::Error;


pub trait Backend{
    fn pause(&mut self);
    fn play(&mut self);
    fn load(&mut self, url: &str);
    fn end(&mut self);
    fn events(&mut self);
}

struct BackendState{
    pub paused: bool,
    pub current_url: String,
    
} 




