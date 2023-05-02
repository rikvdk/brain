fn main() {
    // TODO: remove hardcoding of code and input
    let code = "+++[.-]".as_bytes();
    let input = "abc".as_bytes();
    let mut it = input.iter();

    // TODO: let stack dynamically grow
    let mut stack: [u8; 30_000] = [0; 30_000];

    let mut sp: usize = 0;
    let mut ip: usize = 0;

    while ip < code.len() {
        match code[ip] {
            b'>' => sp += 1,
            b'<' => sp -= 1,
            b'+' => stack[sp] += 1,
            b'-' => stack[sp] -= 1,
            b'.' => print!("{}", stack[sp] as char),
            b',' => stack[sp] = *it.next().expect("bye"),
            b'[' => {
                if stack[sp] == 0 {
                    let mut seen = 1;
                    while seen != 0 {
                        ip += 1;

                        match code[ip] {
                            b'[' => seen += 1,
                            b']' => seen -= 1,
                            _ => (),
                        }
                    }
                }
            }
            b']' => {
                if stack[sp] != 0 {
                    let mut seen = 1;
                    while seen != 0 {
                        ip -= 1;

                        match code[ip] {
                            b']' => seen += 1,
                            b'[' => seen -= 1,
                            _ => (),
                        }
                    }
                }
            }
            _ => (),
        }

        ip += 1;
    }
}
