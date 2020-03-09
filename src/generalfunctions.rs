use std::io;

pub fn GetInput() -> String {
    let mut x = String::new();
    std::io::stdin().read_line(&mut x).expect("Failed");
    x
}
