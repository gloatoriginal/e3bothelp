pub fn MainMenu(GetInput: &dyn Fn() -> String) -> i8 {
    println!("Choose from these decisions:
        1. show saved Users
        2. create new User
        3. change EQ Directory Settings
        4. start Characters
        5. Exit");
    let answer = GetInput().trim().parse::<i8>().unwrap();
    answer
}

