struct Process {
    instructions: String,
    state: State,
    input: String,
    output: String,
    memory: Vec<u8>,
    priority: usize,
}

enum State {
    Running,
    Ready,
    Blocked,
}

fn main() {
    let process = Process {
        instructions: String::from(""),
        state: State::Ready,
        input: String::from(""),
        output: String::from(""),
        memory: Vec::new(),
        priority: 0,
    };
}
