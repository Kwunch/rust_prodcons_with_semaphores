/*
    * SemTrait.rs

    * This file contains the trait for the Semaphore struct.

    * The trait is used to implement the Semaphore struct.
    * Functions For Semaphore:
    *   - wait: Decrements the semaphore.
    *   - signal: Increments the semaphore.

    * For binary semaphores, see BinarySem.rs
    * For counting semaphores, see CountingSem.rs
*/

pub(crate) trait SemTrait {
    fn wait(&mut self);
    fn signal(&mut self);
}