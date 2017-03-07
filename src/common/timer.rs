use time;

pub struct Timer{
    start_time: u64,
    paused: bool,
}

impl Timer{
    pub fn new() -> Self{
        Timer{
            start_time: current_time_ns(),
            paused: false,
        }
    }

    pub fn reset(&mut self){
        self.start_time = current_time_ns()
    }

    pub fn pause(&mut self){
        self.paused = true
    }

    pub fn elapsed(&self)-> u64{
        current_time_ns() - self.start_time
    }

    fn current_time_ns() -> u64 {
        current_time_ns()
    }
}

pub fn current_time_ns() -> u64 {
    time::precise_time_ns()
}

