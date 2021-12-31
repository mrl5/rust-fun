// https://github.com/exercism/rust/tree/630a56517c9015341cdd96233632eda9ad8f44c4/exercises/concept/short-fibonacci
// https://exercism.org/tracks/rust/exercises/short-fibonacci/solutions/mrl5

pub fn create_empty() -> Vec<u8> {
    vec![]
}

pub fn create_buffer(count: usize) -> Vec<u8> {
    vec![0; count]
}

pub fn fibonacci() -> Vec<u8> {
    let mut buffer = create_buffer(5);
    init_fibonacci(&mut buffer);
    calc_fibonacci(&mut buffer);

    buffer
}

fn init_fibonacci(buffer: &mut Vec<u8>) {
    buffer[0] = 1;
    buffer[1] = 1;
}

fn calc_fibonacci(buffer: &mut Vec<u8>) {
    for i in 2 .. buffer.len() {
        buffer[i] = buffer[i-2] + buffer[i-1];
    }
}
