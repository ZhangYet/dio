use std::io;

fn main() -> io::Result<()> {
    loop {
	print!("→ ");

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        dbg!(input);
    }
}
