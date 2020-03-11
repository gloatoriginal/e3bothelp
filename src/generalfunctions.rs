use std::io;

pub fn GetInput() -> String {
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Failed");
    x
}

