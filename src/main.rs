struct Process {
    instructions: String,
    state: State,
    input: String,
    output: String,
    memory: String,
    priority: usize
}

enum State {
    Running,
    Ready,
    Blocked
}



fn main() {
    println!("Hello, world!");
}
