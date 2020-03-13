pub fn MainMenu(GetInput: &dyn Fn() -> String) -> i8 {
    println!("Choose from these decisions:
        1. Show saved Users
        2. Create new User
        3. Start Characters
        4. Exit");
    let answer = GetInput().trim().parse::<i8>().unwrap();
    answer
}

