#![allow(dead_code, unused_variables)]

use std::{
    thread::sleep,
    time::{Duration, SystemTime},
};

use rand::{seq::SliceRandom, Rng};

struct Process {
    process_id: usize,
    state: State,
    input: String,
    output: String,
    memory: Vec<u8>,
    priority: Priority,
    threads: Vec<Thread>,
}

impl Process {
    fn create_thread(&mut self, id: usize, instructions: Vec<String>, state: State) {
        // Creates a thread and pushes it to the threads list
        self.threads.push(Thread {
            process_id: self.process_id,
            id,
            instructions,
            state,
        })
    }

    /// Creates a random process
    fn create_random(id: usize) -> Self {
        // Create a random process
        let mut process = Process {
            process_id: id,
            state: State::Ready,
            input: "".to_string(),
            output: "".to_string(),
            memory: vec![0; 100],
            priority: Priority::random(),
            threads: Vec::new(),
        };

        // Randomly create threads
        let mut rng = rand::thread_rng();
        for id in 0..rng.gen_range(3..=40) {
            // Random instruction length
            let instructions = vec!["".to_string(); rng.gen_range(1..20)];

            // Call the "Create Thread" command
            process.create_thread(id, instructions, State::Ready)
        }

        process
    }
}

struct Thread {
    instructions: Vec<String>,
    id: usize,
    state: State,
    process_id: usize,
}

enum State {
    Running,
    Ready,
    Blocked,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Priority {
    Low,
    High,
    Realtime,
}

impl Priority {
    /// Selects a random priority
    fn random() -> Self {
        *[Priority::Low, Priority::High, Priority::Realtime]
            .choose(&mut rand::thread_rng())
            .unwrap()
    }
}

/// Randomly creates or terminates processes for 10 minutes, printing the actions being taken
fn homework_3() {
    let mut rng = rand::thread_rng();

    let end_time = SystemTime::now() + Duration::from_secs(600);

    let mut processes = Vec::new();

    let mut id = 0;

    while SystemTime::now() < end_time {
        // To keep it from spamming too much
        sleep(Duration::from_millis(100));

        // Randomly decides if it's going to generate or delete, keeping a minimum of 5 processes
        if processes.len() < 5 || rng.gen_bool(0.5) {
            // Create a random process and print out that it's been created
            let process = Process::create_random(id);
            id += 1;
            println!(
                "Creating Process {} with {} threads",
                process.process_id,
                process.threads.len()
            );
            processes.push(process);
        } else {
            // Pick a random process to terminate
            let index = rng.gen_range(0..processes.len());
            let process = processes.swap_remove(index);
            println!(
                "Terminating Process {} with {} threads",
                process.process_id,
                process.threads.len()
            );
        }
    }
}

pub struct CPU {
    process_id: usize,
}

fn homework_4() {
    let mut processes = Vec::new();

    let mut cpu = CPU { process_id: 0 };

    for i in 0..100 {
        processes.push(Process::create_random(i));
    }

    loop {
        // Priority loop behavior
        for priority in [
            Priority::Realtime,
            Priority::High,
            Priority::Realtime,
            Priority::Low,
            Priority::High,
            Priority::Realtime,
        ] {
            // Grab all processes with the given priority
            let process_ids = processes
                .iter()
                .filter(|process| process.priority == Priority::Realtime) // Filter by priority
                .enumerate()
                .map(|(i, _)| i) // We only care about the index of the processes
                .collect::<Vec<_>>();

            for id in process_ids {
                // Update current running process
                processes.get_mut(cpu.process_id).unwrap().state = State::Ready;
                // Switch the cpu process id
                cpu.process_id = id;
                // Update the new process
                processes.get_mut(cpu.process_id).unwrap().state = State::Running;

                // Print out current process
                println!("Running Process {} with Priority {:?}", id, priority);
            }
        }
    }
}

fn main() {
    // homework_3();
    homework_4();
}
