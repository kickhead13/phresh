mod prelude;
mod statics;

use image::RgbImage;
use image::ImageFormat;
use image;
use std::io::{
    prelude::*,
    BufReader
};

use std::collections::HashMap;

pub struct Layer {
    pub imgs: HashMap<String, usize>
}

pub struct Lexer<'b, B> {
    pub buf_reader: &'b mut BufReader<B>,
    pub layers: Vec<Layer>,
    pub imgs: HashMap<String, usize>,
    pub memory: Vec<RgbImage>
}

impl<'b, B> Lexer<'b, B> where B: std::io::Read {
    pub fn new(buf_reader: &'b mut BufReader<B>) -> Self {
        Self {
            buf_reader: buf_reader,
            imgs: HashMap::<String, usize>::new(),
            layers: Vec::new(),
            memory: Vec::new()
        }
    }
    
    pub fn next_line(self: &mut Self) -> Option<String> {
        let mut line = String::new();
        match self.buf_reader.read_line(&mut line) {
            Ok(len) => {
                if len == 0 {
                    return None;
                }
                _ = line.pop();
                Some(line.clone())
            },
            Err(_) => None
        }

    }

    pub fn img_command(self: &mut Self, words: Vec<&str>) {
        if words.len() < 3 {
            eprintln!("[ERROR] Not enough params for img command");
            std::process::exit(3);
        }
        
        if prelude::word_type(words[1].to_string()) != prelude::WordType::Variable 
        || prelude::word_type(words[2].to_string()) != prelude::WordType::StringValue  {
            eprintln!("[ERROR] Wrong types for params in img command");
            std::process::exit(3);
        }
        
        let mut filename = words[2].to_string();
        _ = filename.remove(0);
        _ = filename.pop();

        match image::open(&filename) {
            Ok(img) => {
                self.memory.push(img.into_rgb8());
                self.imgs.insert(
                    words[1].to_string().clone(),
                    self.memory.len() - 1
                );
            },
            Err(_) => {
                eprintln!("[ERROR] Failed to open {filename} image.");
                std::process::exit(4);
            }
        }
    }

    pub fn save_command(self: &Self, words: Vec<&str>) {
        if words.len() < 4 {
            eprintln!("[ERROR] Not enough params for save command");
            std::process::exit(3);
        }
        
        if prelude::word_type(words[1].to_string()) != prelude::WordType::Variable 
        || prelude::word_type(words[2].to_string()) != prelude::WordType::StringValue  
        || prelude::word_type(words[3].to_string()) != prelude::WordType::Extension {
            eprintln!("[ERROR] Wrong types for params in img command");
            std::process::exit(3);
        }
        
        let mut filename = words[2].to_string();
        _ = filename.remove(0);
        _ = filename.pop();

        match self.imgs.get(words[1]) {
            Some(&index) => {
                self.memory[index].save_with_format(filename, ImageFormat::from_extension(words[3]).expect("FAIL"));
            },
            None => {
                eprintln!("[ERROR] {} is not a variable", words[1]);
                        std::process::exit(5);
            }
        }
    }

    pub fn layer_command(self: &Self, words: Vec<&str>) {
        println!("layer");
    }

    pub fn interpret(self: &mut Self, line: String) {
        let words = line.split(" ").collect::<Vec<_>>();
        match prelude::word_type(words[0].to_string()) {
            prelude::WordType::ImgCommand => {
                self.img_command(words);
            },
            prelude::WordType::SaveCommand => {
                self.save_command(words);
            },
            prelude::WordType::LayerCommand => {
                self.layer_command(words);
            },
            prelude::WordType::Variable => {
                println!("VARIABLE");
            },
            _ => {
                println!("WRONG");
            }
        }
    }

    pub fn start(self: &mut Self) {
        while let Some(line) = self.next_line() {
            self.interpret(line);
        }   
    }
}


