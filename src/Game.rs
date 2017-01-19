use std::sync::{Arc, Mutex, Once, ONCE_INIT};
use std::{mem, thread};

#[derive(Clone)]
pub struct Game {
	pub inner: Arc<Mutex<u8>>,
	_frameLastFPS: u64,
	_lastFrameTime: u64,
	_frameCount: i32,
	_frameRate: i32,
}

impl Game {

	pub fn instance() -> Game {
        // Initialize it to a null value
        static mut SINGLETON: *const Game = 0 as *const Game;
        static ONCE: Once = ONCE_INIT;

        unsafe {
            ONCE.call_once(|| {
                // Make it
                let instance = Game {
                    inner: Arc::new(Mutex::new(0)),
                    _frameCount: 0,
                    _frameLastFPS: 0,
                    _frameRate: 0,
                    _lastFrameTime: 0u64,
                };

                // Put it in the heap so it can outlive this call
                SINGLETON = mem::transmute(Box::new(instance));
            });

            // Now we give out a copy of the data that is safe to use concurrently.
            (*SINGLETON).clone()
        }
    }

	pub fn frame(&mut self)
	{
		if self._lastFrameTime == 0u64 {
			self._lastFrameTime = Game::getGameTime();
		}

		let frameTime = Game::getGameTime();
		let elapsedTime = (frameTime - self._lastFrameTime);
        self._lastFrameTime = frameTime;

        Game::update(elapsedTime);
        Game::render(elapsedTime);

		self._frameCount += 1;
		if (Game::getGameTime() - self._frameLastFPS) >= 1000
        {
            self._frameRate = self._frameCount;
            self._frameCount = 0;
            self._frameLastFPS = Game::getGameTime();
        }

	}

	pub fn getGameTime() -> u64 {
		use time;

		return time::precise_time_ns() ;
	}

	fn update(elapsedTime:u64)
	{

	}

	fn render(elapsedTime:u64)
	{

	}
	
}

