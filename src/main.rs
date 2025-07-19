use std::error::Error;
use std::fs::File;
use std::io::BufReader;

mod io;
mod lexer;

fn main() -> Result<(), Box<dyn Error>> {
    let phresh_args: Vec<String> = std::env::args().collect();
    if phresh_args.len() < 2 {
        eprintln!(" Usage: phresh <filename>");
        eprintln!("   <filename> is NOT optional");
    }

    let phresh_filename = phresh_args[1].clone();

    //TODO: FIX THIS (NO ERROR OUTPUT)
    let phresh_file = File::open(phresh_filename)?;
    let mut bufreader = BufReader::new(phresh_file);

    let mut lex = lexer::Lexer::new(&mut bufreader);
    
    lex.start();

    Ok(())
}
