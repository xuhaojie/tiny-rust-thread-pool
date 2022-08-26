use std::rc::Rc;
use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::{Arc, Mutex};
type Task = dyn FnOnce() + Send;

enum Message {
	NewTask(Box<Task>),
	Terminate,
}

struct Worker{
	id: usize,
	handle: Option<thread::JoinHandle<()>>
}

impl Worker {
	fn new(id: usize, receiver: Arc<Mutex<Receiver<Message>>>) -> Self {
		let handle = thread::spawn(move || {
			loop {
				let msg = receiver.lock().unwrap().recv().unwrap();
				match msg {
					Message::NewTask(task) => {
						task();
					},
					Message::Terminate => break,
				}
			}
		});
		Worker { id, handle: Some(handle) }
	}
}


struct Threadpool{
	workers: Vec<Worker>,
	sender: Sender<Message>,
}

impl Threadpool {
	fn new(size: usize) -> Self {
		let mut workers = Vec::with_capacity(size);
		let (sender, receiver) = channel::<Message>();
		let receiver = Arc::new(Mutex::new(receiver));
		for id in 0..size {
			let worker = Worker::new(id, receiver.clone());
			workers.push(worker);
		}
		Threadpool { workers, sender}
	}

	fn execute<F>(&mut self, f: F)
	where F: FnOnce() + Send + 'static, {
		self.sender.send(Message::NewTask(Box::new(f))).unwrap();
	}
}


impl Drop for Threadpool {
	fn drop(&mut self) {
		//for worker in &mut self.workers {
		for _ in &self.workers {
			self.sender.send(Message::Terminate).unwrap();
		}
		for worker in &mut self.workers {
			if let Some(h) = worker.handle.take() {
				h.join().unwrap();
			}
		}
	}
}

fn main() {
	// let thread = thread::spawn(|| {
// 		println!("Hello, world!");
// 	});
// 	thread.join().unwrap();
	const THREADS : usize = 4;
	let mut thread_pool = Threadpool::new(THREADS);
	thread_pool.execute(move || {
			println!("Hello, world!");
		}
	);
}


