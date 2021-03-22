use std::io::{self, Write};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();
    
    let mut input = String::new();
    let mut e = dio::Env::default();
    
    loop {
	write!(stdout, "→ ")?;
	stdout.flush()?;

        let mut input = String::new();
        stdin.read_line(&mut input)?;

	match run(input.trim(), &mut e) {
	    Ok(Some(val)) => writeln!(stdout, "{}", val)?,
	    Ok(None) => {}
	    Err(msg) => writeln!(stderr, "{}", msg)?,
	}
    }
}

fn run(input: &str, env: &mut dio::Env) -> Result<Option<dio::Val>, String> {
    let parse = dio::parse(input).map_err(|msg| format!("Parse error: {}", msg))?;

    let evaluated = parse
	.eval(env)
	.map_err(|msg| format!("Evaluation error: {}", msg))?;

    if evaluated == dio::Val::Unit {
	Ok(None)
    } else {
	Ok(Some(evaluated))
    }
}
