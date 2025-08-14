/// A chat system info updater
pub trait SystemInfo {
    /// creates a new instance
    fn new() -> Box<Self>
    where Self: Sized;
    
    /// updates a system info:
    fn update(&mut self) -> String;
}
