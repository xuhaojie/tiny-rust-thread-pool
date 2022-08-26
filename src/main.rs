use std::thread;

struct Task;

enum Message {
	NewTask(Task),
	Terminate,
}

struct Worker{
	id: usize,
	handle: Option<thread::JoinHandle<()>>
}

impl Worker {
	fn new(id: usize) -> Self {
		let handle = thread::spawn(|| {
			println!("Hello, world!");
		});
		Worker { id, handle: Some(handle) }
	}
}


struct Threadpool{
	workers: Vec<Worker>,
}

impl Threadpool {
	fn new(size: usize) -> Self {
		let mut workers = Vec::with_capacity(size);
		for id in 0..size {
			let worker = Worker::new(id);
			workers.push(worker);
		}
		Threadpool { workers }
	}
}


impl Drop for Threadpool {
	fn drop(&mut self) {
		//for worker in &mut self.workers {
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
	let thread_pool = Threadpool::new(THREADS);

}


