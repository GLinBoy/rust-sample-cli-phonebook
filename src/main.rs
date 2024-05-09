use std::io;
use std::io::Write;

fn main() {
    println!("1. Add new entry");
    println!("2. Explorer entries");
    println!("3. Delete an entry");
    println!("4. Quite");
    println!("-----------------------------");
    print!("Select an option: ");
    io::stdout().flush().unwrap();

    let mut line = String::new();

    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read from stdin.");

    let trimmed = line.trim();
    match trimmed.parse::<u32>() {
        Ok(i) => match i {
            1 => println!("Selected to add a new entry"),
            2 => println!("Selected to explorers entries"),
            3 => println!("Selected to delete an entry"),
            4 => println!("Selected to exit the application"),
            _ => println!("Wrong entry! try again (1-4).")
        },
        Err(..) => println!("This was not an integer: {}", trimmed),
    };
}
