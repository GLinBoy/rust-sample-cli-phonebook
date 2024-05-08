use std::io;

fn main() {
    println!("1. Add new entry");
    println!("2. Explorer entries");
    println!("3. Delete an entry");
    println!("4. Quite");
    println!("-----------------------------");
    print!("Select an option: ");

    let mut line = String::new();

    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read from stdin.");

    let trimmed = line.trim();
    match trimmed.parse::<u32>() {
        Ok(i) => println!("Your integer input: {}", i),
        Err(..) => println!("This was not an integer: {}", trimmed),
    };
}
