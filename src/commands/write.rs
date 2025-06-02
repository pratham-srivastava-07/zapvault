use std::io::{self, Write};


pub fn handle_write(tag: Option<String>) {

    let tag = match tag {
        Some(t) => t,
        None => {
            print!("Enter a tag: ");
            io::stdout().flush().unwrap(); // ensure prompt is shown
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read tag");
            input.trim().to_string()

        }
    };
    let mut content = String::new();
    print!("Enter content: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut content).expect("Failed to read content");

    println!("The tag is: {}", tag);
    println!("The content received is: {}", content.trim());
}