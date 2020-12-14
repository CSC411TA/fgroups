use std::io;
use std::str;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let parts: Vec<&str> = input.splitn(2, |c|
    	c == ' ' || c == '\t').collect();
    println!("Fprint: {}, Name: {}", parts[0], parts[1]);
    Ok(())
}
