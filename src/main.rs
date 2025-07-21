use std::error::Error;
use std::fs::File;
use std::io::BufReader;

mod io;
mod lexer;

enum StdinFile {
    Stdin(std::io::Stdin),
    File(File)
}

fn main() -> Result<(), Box<dyn Error>> {
    let phresh_args: Vec<String> = std::env::args().collect();
    let phresh_file = match phresh_args.len() < 2 {
        false => {
            let phresh_filename = phresh_args[1].clone();

            //TODO: FIX THIS (NO ERROR OUTPUT)
            StdinFile::File(match File::open(phresh_filename) {
                Ok(file) => file,
                Err(_) => {
                    std::process::exit(1);
                },
            })
        },
        true => {
            StdinFile::Stdin(std::io::stdin())
        }
    }; 

    match phresh_file {
        StdinFile::Stdin(stdin) => {
            println!(" dropped to phresh console:");
            let mut bufreader = BufReader::new(stdin);
            let mut lex = lexer::Lexer::new(&mut bufreader);
            lex.start();
        },
        StdinFile::File(file) => {
            let mut bufreader = BufReader::new(file);
            let mut lex = lexer::Lexer::new(&mut bufreader);
            lex.start();       
        },
    };

    Ok(())
}
