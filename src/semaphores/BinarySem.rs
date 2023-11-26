/*
    * BinarySem.rs

    * This module contains the implementation of a binary semaphore.
    * A binary semaphore is a semaphore that can only have two states:
    * 0 or 1. It is used to implement mutual exclusion.
    *

    * For counting semaphores, see CountingSem.rs
*/

use std::sync::Mutex;
use std::thread::*;
use super::SemTrait::SemTrait;


/*
    * BinarySem

    * This struct implements the SemTrait trait.
    * It is used to implement a binary semaphore.

    * Contains the following:
    *   - value: The value of the semaphore.
    *   - lock: Mutex used to lock the semaphore. (Instead of spinlock)
    *   - queue: Queue containing threads that are blocked on the semaphore.
*/
pub(crate) struct Semaphore {
    value: i32, // The value of the semaphore
    lock: Mutex<()>, // Mutex used to lock the semaphore
    queue: Vec<Thread>, // Queue containing threads that are blocked on the semaphore
}

impl Semaphore {
    pub(crate) fn new() -> Self {
        let lock = Mutex::new(());
        let queue = Vec::new();
        Semaphore {
            value: 1,
            lock,
            queue,
        }
    }
}

impl SemTrait for Semaphore {
    fn wait(&mut self) {
        /*
            * wait function

            * Mutex lock the semaphore.
            * If value is 1, set value to 0. Then unlock the semaphore.
            * If value is 0, add to process queue and block.
                - Unlock semaphore before blocking.
        */

        let lock = self.lock.lock().unwrap();

        if self.value == 1 {
            self.value = 0;
        } else {
            self.queue.push(current()); // Add current thread to queue
            drop(lock); // Drop lock before blocking
            park(); // Block thread

            return
        }

        // Mutex unlock the semaphore after if statement
        drop(lock);
    }

    fn signal(&mut self) {
        /*
            * signal function

            * Mutex lock the semaphore.
            * If queue is empty, set value to 1.
            * If queue is not empty, unblock the first process in the queue.

            * Mutex unlock the semaphore.
         */

        let lock = self.lock.lock().unwrap();

        if self.queue.is_empty() {
            self.value = 1;
        } else {
            // Unblock first process in queue
            let thread = self.queue.remove(0);
            thread.unpark();
        }

        // Mutex unlock the semaphore after if statement
        drop(lock);
    }
}
