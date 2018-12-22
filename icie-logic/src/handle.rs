use super::{Directory, Impulse, Reaction, ICIE};
use std::{
	panic, sync::{
		mpsc::{self, Receiver, Sender}, Mutex
	}, thread::{self, sleep}, time::Duration
};

pub struct Handle {
	input: Mutex<Sender<Impulse>>,
	output: Mutex<Receiver<Reaction>>,
}
impl Handle {
	pub fn spawn() -> Handle {
		let (es, er) = mpsc::channel();
		let (is, ir) = mpsc::channel();
		let es2 = es.clone();
		let is2 = Mutex::new(is.clone());
		panic::set_hook(Box::new(move |info| {
			if let Ok(is2) = is2.lock() {
				let _ = is2.send(Reaction::ErrorMessage { message: info.to_string() });
			}
			loop {
				sleep(Duration::from_secs(1));
			}
		}));

		thread::spawn(move || {
			ICIE {
				input: er,
				output: is,
				input_sender: es2,
				directory: Directory::new_empty(),
			}
			.main_loop()
		});
		Handle {
			input: Mutex::new(es),
			output: Mutex::new(ir),
		}
	}

	pub fn send(&self, message: Impulse) {
		self.input.lock().unwrap().send(message).unwrap()
	}

	pub fn recv(&self) -> Reaction {
		self.output.lock().unwrap().recv().unwrap()
	}
}
