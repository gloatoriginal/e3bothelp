
pub fn MainMenu(GetInput: &dyn Fn() -> String) -> i8 {
    println!("Choose from these decisions:
        1. Show Active Characters
        2. Create New Character
        3. Exit");
    let mut answer = GetInput().trim().parse::<i8>().unwrap();
    answer
}
