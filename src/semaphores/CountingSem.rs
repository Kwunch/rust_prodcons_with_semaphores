/*
    @Author: Kwunch

    * CountingSem.rs

    * Implementation of a counting semaphore.
    * This is used for the producer-consumer problem.

    * Contains the following:
        - value: The value of the semaphore.
        - lock: Mutex used to lock the semaphore. (Instead of spinlock)
        - queue: Queue containing threads that are blocked on the semaphore.
*/
use std::thread::*;
use std::sync::Mutex;
use super::SemTrait::SemTrait;

/*
    * CountingSem

    * This struct implements the SemTrait trait.
    * It is used to implement a counting semaphore.
*/
pub(crate) struct Semaphore {
    value: i32, // The value of the semaphore
    lock: Mutex<()>, // Mutex used to lock the semaphore
    queue: Vec<Thread>, // Queue containing threads that are blocked on the semaphore
}

impl Semaphore {
    pub(crate) fn new(value: i32) -> Self {
        let lock = Mutex::new(());
        let queue = Vec::new();
        Semaphore {
            value,
            lock,
            queue,
        }
    }
}

impl SemTrait for Semaphore {

    fn wait(&mut self) {
        /*
            * Wait for counting semaphore.
        */
        let lock = self.lock.lock().unwrap();

        self.value -= 1;
        if self.value < 0 {
            self.queue.push(current());
            drop(lock);
            park();

            return
        }
        drop(lock);
    }

    fn signal(&mut self) {
        /*
            * Signal for counting semaphore.
        */
        let lock = self.lock.lock().unwrap();

        self.value += 1;
        if self.value <= 0 {
            let thread = self.queue.remove(0);
            thread.unpark();
        }
        drop(lock);
    }
}
