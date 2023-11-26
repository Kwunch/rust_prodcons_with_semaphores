/*
    @Author: Kwunch

    * Rust implementation of producer-consumer problem

    * Uses both binary and counting semaphores. (based off user input)

    * This program will create a producer and consumer process.
        * Number of producer and consumer processes is based off user input.

    * This program will also create a BUFFER.
        * Size of BUFFER is based off user input.

    * The producer will increment 'total' by 1 and add it to the BUFFER.
    * The consumer will just read from the BUFFER.

    * Runs until user hits Enter.
*/

use std::{
    env,
    process::exit,
    thread::{spawn, JoinHandle},
};

use semaphores::{
    BinarySem::Semaphore as BinarySem,
    CountingSem::Semaphore as CountingSem,
    SemTrait::SemTrait,
};

pub(crate) mod semaphores;

// Vars: in, out, total
static mut IN: usize = 0;
static mut OUT: usize = 0;
static mut TOTAL: u64 = 0;

// Vars: semaphores, buffer
static mut SEMMUTEX: Option<Box<dyn SemTrait>> = None;
static mut SEMEMPTY: Option<Box<dyn SemTrait>> = None;
static mut SEMFULL: Option<Box<dyn SemTrait>> = None;
static mut BUFFER: Vec<u64> = Vec::new();

// Vars: end_flag (used to determine when to end program)
static mut END_FLAG: bool = false;

// Vars: Alphabetic characters (used for producer/consumer id)
static IDS: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
    'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
];

fn main() {
    let is_binary: bool; // Used to determine which prod/con to call
    let args: Vec<String> = env::args().collect(); // Get args

    // Create threads vector
    let mut threads: Vec<JoinHandle<()>> = Vec::new();

    // Get BUFFER size
    let buffer_size: usize = match args[args.len() - 1].parse::<usize>() {
        Ok(value) => value,
        Err(_) => {
            println!("Error: Invalid BUFFER size.");
            exit(1);
        }
    };

    /*
        ** UNSAFE **
        * Create semaphore based off user input.

        * If user input is invalid, exit.

        * If counting semaphore:
            - Create 3 counting semaphores. First is mutex, second is empty, third is full.
            - Empty is initialized to BUFFER size.
        * If binary semaphore:
            - Create binary semaphore. (No user input value)
    */
    unsafe {
        SEMMUTEX = match args[1].to_lowercase().as_str() {
            "-c" if args.len() == 5 => {
                /*
                    * Create 3 counting semaphores.
                    * First is mutex, second is empty, third is full.
                    * Empty is initialized to BUFFER size.

                    * Set is_binary to false.
                    * SEMMUTEX is initialized from the last match statement.
                */
                is_binary = false;
                SEMEMPTY = Some(Box::new(CountingSem::new(buffer_size as i32)));
                SEMFULL = Some(Box::new(CountingSem::new(0)));
                Some(Box::new(CountingSem::new(1)))
            }
            "-b" if args.len() == 5 => {
                /*
                    * Create binary semaphore.

                    * Set is_binary to true.
                */
                is_binary = true;
                Some(Box::new(BinarySem::new()))
            }
            _ => {
                panic!("Error: Invalid arguments.");
            }
        };
    }

    // UNSAFE: Create BUFFER
    unsafe { BUFFER = vec![0; buffer_size]; }

    // Get number of producers
    let num_producers: usize = match args[args.len() - 3].parse::<usize>() {
        Ok(value) => value,
        Err(_) => {
            println!("Error: Invalid number of producers.");
            exit(1);
        }
    };

    // Get number of consumers
    let num_consumers: usize = match args[args.len() - 2].parse::<usize>() {
        Ok(value) => value,
        Err(_) => {
            println!("Error: Invalid number of consumers.");
            exit(1);
        }
    };

    // Create producer threads
    for i in 0..num_producers {
        /*
            * Create producer threads.
            * If is_binary is true, call bin_producer. (binary semaphore)
            * If is_binary is false, call cou_producer. (counting semaphore)

            * Each thread pushes itself to the threads vector.
        */
        if is_binary {
            threads.push(spawn(move || unsafe {
                bin_producer(IDS[i], &buffer_size);
            }));
        } else {
            threads.push(spawn(move || unsafe {
                cou_producer(IDS[i], &buffer_size);
            }));
        }
    }

    // Create consumer threads
    for i in 0..num_consumers {
        /*
            * Create consumer threads.
            * If is_binary is true, call bin_consumer. (binary semaphore)
            * If is_binary is false, call cou_consumer. (counting semaphore)

            * Each thread pushes itself to the threads vector.
        */
        if is_binary {
            threads.push(spawn(move || unsafe {
                bin_consumer(IDS[i], &buffer_size);
            }));
        } else {
            threads.push(spawn(move || unsafe {
                cou_consumer(IDS[i], &buffer_size);
            }));
        }
    }

    // If user presses enter, set END_FLAG to true
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    unsafe { END_FLAG = true; }

    // Wait for threads to finish
    for thread in threads {
        thread.join().unwrap();
    }

    println!("Enter pressed and threads finished...");
}

unsafe fn bin_producer(id: char, buffer_size: &usize) {
    /*
        * Producer function for binary semaphore.

        * Producer will increment 'total' by 1 and add it to the BUFFER.
        * Producer will print what it added to the BUFFER.
    */
    let mutex = SEMMUTEX.as_mut().unwrap();
    while !END_FLAG {
        mutex.wait();

        // Critical section
        BUFFER[IN] = TOTAL;
        TOTAL += 1;
        println!("Producer {} Produced: {}", id, BUFFER[IN]);
        IN = (IN + 1) % *buffer_size;

        mutex.signal();
    }
}

unsafe fn bin_consumer(id: char, buffer_size: &usize) {
    /*
        * Consumer function for binary semaphore.

        * Consumer will read from BUFFER.
        * Consumer will print what it read from BUFFER.
    */
    let mutex = SEMMUTEX.as_mut().unwrap();
    while !END_FLAG {
        mutex.wait();

        // Critical section
        println!("Consumer {} Consumed: {}", id, BUFFER[OUT]);
        OUT = (OUT + 1) % *buffer_size;

        mutex.signal();
    }
}

unsafe fn cou_producer(id: char, buffer_size: &usize) {
    /*
        * Producer function for counting semaphore.

        * Producer will increment 'total' by 1 and add it to the BUFFER.
        * Producer will print what it added to the BUFFER.
    */
    let mutex = SEMMUTEX.as_mut().unwrap();
    let empty = SEMEMPTY.as_mut().unwrap();
    let full = SEMFULL.as_mut().unwrap();
    while !END_FLAG {
        empty.wait();
        mutex.wait();

        // Critical section
        BUFFER[IN] = TOTAL;
        TOTAL += 1;
        println!("Producer {} Produced: {}", id, BUFFER[IN]);
        IN = (IN + 1) % *buffer_size;

        mutex.signal();
        full.signal();
    }
}

unsafe fn cou_consumer(id: char, buffer_size: &usize) {
    /*
        * Consumer function for counting semaphore.

        * Consumer will read from BUFFER.
        * Consumer will print what it read from BUFFER.
    */
    let mutex = SEMMUTEX.as_mut().unwrap();
    let empty = SEMEMPTY.as_mut().unwrap();
    let full = SEMFULL.as_mut().unwrap();
    // Loop until user presses ctrl-c
    while !END_FLAG {
        full.wait();
        mutex.wait();

        // Critical section
        println!("Consumer {} Consumed: {}", id, BUFFER[OUT]);
        OUT = (OUT + 1) % *buffer_size;

        mutex.signal();
        empty.signal();
    }
}
