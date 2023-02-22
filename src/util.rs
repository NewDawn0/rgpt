
/* fn confirm: get confirmation from user
 * @RVAL: bool*/
fn confirm() -> bool {
    let mut input = String::new();

    loop {
        print!("{}>{} Confirm run: [y/N]: ", COLOURS.red, COLOURS.reset);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => {
                println!("Invalid input. Please enter 'y' or 'n'");
                input.clear();
            }
        }
    }
}
