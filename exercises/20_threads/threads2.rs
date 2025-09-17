// Building on the last exercise, we want all of the threads to complete their
// work. But this time, the spawned threads need to be in charge of updating a
// shared value: `JobStatus.jobs_done`

use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration
};

struct JobStatus {
    jobs_done: u32,
}

fn main() {
    // TODO: `Arc` isn't enough if you want a **mutable** shared state.
    let status = Arc::new(Mutex::new(JobStatus { jobs_done: 0 }));

    let mut handles = Vec::new();
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));

            // TODO: You must take an action before you update a shared value.
            let mut status = status_shared.lock().unwrap();
            status.jobs_done += 1;
        });
        handles.push(handle);
    }

    // Waiting for all jobs to complete.
    for handle in handles {
        handle.join().unwrap();
    }

    // TODO: Print the value of `JobStatus.jobs_done`.
    println!("Jobs done: {}", status.lock().unwrap().jobs_done);
}

/*
Arc<Mutex<T>> allows us to:

1. Share access to data between threads (Arc);
2. Ensure mutually exclusive access to this data (Mutex).

status_shared.lock().unwrap() — takes the lock before accessing the data.
*/

/*
Alternative:

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;

fn main() {
    let jobs_done = Arc::new(AtomicUsize::new(0));

    let mut handles = Vec::new();
    for _ in 0..10 {
        let jobs_shared = Arc::clone(&jobs_done);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            jobs_shared.fetch_add(1, Ordering::SeqCst); // атомарное увеличение
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Всего завершено задач: {}", jobs_done.load(Ordering::SeqCst));
}
*/