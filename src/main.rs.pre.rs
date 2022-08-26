use std::sync::{Arc, Mutex,mpsc::{channel,Sender, Receiver}};
use std::thread;
type Task = Box<dyn FnOnce() + Send>;

enum Message {
	NewTask(Task),
	Terminate,
}

struct Worker{
	id: usize,
	thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
	fn new(id: usize, receiver: Arc<Mutex<Receiver<Message>>>) -> Self {
		let thread = thread::spawn(move || {
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
		Worker{id,  thread: Some(thread)}
	}
}

struct Threadpool{
	size: usize,
	workers: Vec<Worker>,
	sender: Sender<Message>,
}

impl Threadpool {
	fn new(size: usize) -> Threadpool {
		let (sender, receiver) = channel::<Message>();
		let receiver = Arc::new(Mutex::new(receiver));
		let mut workers = Vec::new();
		for id in 0..size {
			workers.push(Worker::new(id, receiver.clone()));
		}
		Threadpool { size, workers,sender}
	}

	fn execute<F>(&mut self, f: F) 
	where F: FnOnce() -> (),
	F: Send + 'static,
	{
		self.sender.send(Message::NewTask(Box::new(f)));
	}
}

impl Drop for Threadpool {
	fn drop(&mut self) {
		for worker in &self.workers {
			self.sender.send(Message::Terminate);
		}

		for worker in &mut self.workers {
			worker.thread.take().unwrap().join().unwrap();
		}		
	}
}

fn main() {
	let mut thread_pool = Threadpool::new(4);
	thread_pool.execute(move||{println!("Hello, world!");});

}


