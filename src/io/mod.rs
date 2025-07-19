use std::io::{
    prelude::*,
    BufReader
};

#[allow(dead_code)]
pub fn phresh_line<B>(
    bufreader: &mut BufReader<B>
) -> String where B: std::io::Read {
    let mut line = String::new();
    match bufreader.read_line(&mut line) {
        Ok(_) => {
            return line.clone();
        },
        Err(_) => {
            return "".to_string();
        }
    }
}
