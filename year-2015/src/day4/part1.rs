use std::thread::{self, JoinHandle};
use std::sync::mpsc;

const INPUT: &str = "bgvyzdsv";

struct Worker {
    thread: Option<JoinHandle<()>>,
    tx: mpsc::Sender<Option<u64>>
}

impl Worker {
    fn spawn() -> Self {
        let (tx, rx) = mpsc::channel();

        let thread = thread::spawn(move || {
            while let Ok(msg) = rx.recv() {
                match msg {
                    Some(_) => (),
                    None => break
                }
            }
        });

        Self {
            thread: Some(thread),
            tx
        }
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        if let Some(thread) = self.thread.take() {
            let _ = self.tx.send(None);
            let _ = thread.join();
        }
    }
}



struct ThreadPool<const N: usize> {
    workers: Vec<Worker>
}

impl<const N: usize> ThreadPool<N> {
    fn new() -> Self {
        let mut workers = Vec::with_capacity(N);

        for _ in 0..N {
            workers.push(Worker::spawn());
        }

        Self {
            workers
        }
    }
}

fn main() {
    let tp = ThreadPool::<12>::new();
}
