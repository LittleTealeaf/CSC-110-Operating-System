#![allow(dead_code, unused_variables)]

struct Process {
    process_id: usize,
    state: State,
    input: String,
    output: String,
    memory: Vec<u8>,
    priority: usize,
    threads: Vec<Thread>,
}

struct Thread {
    instructions: String,
    id: usize,
    state: State,
    process_id: usize,
}

impl Process {
    fn create_thread(&mut self, id: usize, instructions: String, state: State) {
        self.threads.push(Thread {
            process_id: self.process_id,
            id,
            instructions,
            state,
        })
    }
}

enum State {
    Running,
    Ready,
    Blocked,
}

fn main() {
    let process = Process {
        process_id: 0,
        state: State::Ready,
        input: String::from(""),
        output: String::from(""),
        memory: Vec::new(),
        priority: 0,
        threads: vec![],
    };
}
