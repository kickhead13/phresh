mod prelude;
mod statics;

use image::RgbImage;
use image::ImageFormat;
use image;
use std::io::{
    prelude::*,
    BufReader
};

use rand::prelude::*;
use reqwest;
use std::fs::File;

use std::collections::HashMap;

pub struct Lexer<'b, B> {
    pub buf_reader: &'b mut BufReader<B>,
    pub imgs: HashMap<String, usize>,
    pub memory: Vec<RgbImage>
}

impl<'b, B> Lexer<'b, B> where B: std::io::Read {
    pub fn new(buf_reader: &'b mut BufReader<B>) -> Self {
        Self {
            buf_reader: buf_reader,
            imgs: HashMap::<String, usize>::new(),
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

        if filename.starts_with("https://") || filename.starts_with("http://") {
            let rngfilename = format!("{}{}.jpg", words[1], rand::rng().random::<i32>());
            if let Ok(response) = reqwest::blocking::get(filename.clone()) {
                if let (Ok(bytes), Ok(mut file)) = (response.bytes(), File::create(&rngfilename)) {
                    match file.write_all(&bytes) {
                        Ok(_) => {
                            filename = rngfilename.to_string();
                        },
                        Err(_) => {
                            eprintln!("[ERROR] Couldn't get {filename}...");
                            std::process::exit(6);
                        }
                    }    
                } else {
                    eprintln!("[ERROR] Couldn't get {filename}...");
                    std::process::exit(6);
                }
            } else {
                eprintln!("[ERROR] Couldn't get {filename}...");
                std::process::exit(6);
            }
        }

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
                _ = self.memory[index].save_with_format(filename, ImageFormat::from_extension(words[3]).expect("FAIL"));
            },
            None => {
                eprintln!("[ERROR] {} is not a variable", words[1]);
                        std::process::exit(5);
            }
        }
    }

    pub fn layer_command(self: &mut Self, words: Vec<&str>) {
        if words.len() < 6 {
            eprintln!("[ERROR] Not enough params for layer command");
            std::process::exit(3);
        }
        
        if prelude::word_type(words[1].to_string()) != prelude::WordType::Variable 
        || prelude::word_type(words[2].to_string()) != prelude::WordType::Variable 
        || prelude::word_type(words[3].to_string()) != prelude::WordType::NumValue
        || prelude::word_type(words[4].to_string()) != prelude::WordType::NumValue
        || prelude::word_type(words[5].to_string()) != prelude::WordType::NumValue {
            eprintln!("[ERROR] Wrong types for params in img command");
            std::process::exit(3);
        }
        
        let pos_x = words[3].to_string().parse::<i32>().unwrap();
        let pos_y = words[4].to_string().parse::<i32>().unwrap();
        let opacity = words[5].to_string().parse::<u32>().unwrap();
            
        if let (Some(&indexv1), Some(&indexv2)) = (self.imgs.get(words[1]), self.imgs.get(words[2])) {
            let widthv1 = self.memory[indexv1].width();
            let heightv1 = self.memory[indexv1].height();
            let widthv2 = self.memory[indexv2].width();
            let heightv2 = self.memory[indexv2].height();
            let cloned_v2_mem = self.memory[indexv2].clone();
            for xi in (std::cmp::max(pos_x, 0) as u32)..(std::cmp::min(widthv1 as i32, widthv2 as i32 + pos_x) as u32) {
                for yi in (std::cmp::max(pos_y, 0) as u32)..(std::cmp::min(heightv1 as i32, heightv2 as i32 + pos_y) as u32) {
                    let pixelv1 = self.memory[indexv1].get_pixel_mut(xi as u32, yi as u32);
                    let pixelv2 = cloned_v2_mem.get_pixel((xi as i32 - pos_x) as u32, (yi as i32 - pos_y) as u32);
                    pixelv1.0[0] = ((u32::from(pixelv1.0[0]) * (100 - opacity))/100 + (u32::from(pixelv2.0[0]) * opacity) / 100) as u8;
                    pixelv1.0[1] = ((u32::from(pixelv1.0[1]) * (100 - opacity))/100 + (u32::from(pixelv2.0[1]) * opacity) / 100) as u8;
                    pixelv1.0[2] = ((u32::from(pixelv1.0[2]) * (100 - opacity))/100 + (u32::from(pixelv2.0[2]) * opacity) / 100) as u8;
                }
            }
        } else {
            eprintln!("[ERROR] {} is not a variable", words[1]);
            std::process::exit(5);
        }
    }
        
    pub fn downscale_command(self: &mut Self, words: Vec<&str>) {
        if words.len() < 7 {
            eprintln!("[ERROR] Not enough params for downsize command");
            std::process::exit(3);
        }
        
        if prelude::word_type(words[1].to_string()) != prelude::WordType::Variable 
        || prelude::word_type(words[2].to_string()) != prelude::WordType::Variable 
        || prelude::word_type(words[3].to_string()) != prelude::WordType::NumValue
        || prelude::word_type(words[4].to_string()) != prelude::WordType::NumValue
        || prelude::word_type(words[5].to_string()) != prelude::WordType::NumValue 
        || prelude::word_type(words[6].to_string()) != prelude::WordType::NumValue {
            eprintln!("[ERROR] Wrong types for params in img command");
            std::process::exit(3);
        }
        
        let pos_x = words[3].to_string().parse::<i32>().unwrap();
        let pos_y = words[4].to_string().parse::<i32>().unwrap();
        let opacity = words[5].to_string().parse::<u32>().unwrap();
        let scale = words[6].to_string().parse::<u32>().unwrap();   

        if let (Some(&indexv1), Some(&indexv2)) = (self.imgs.get(words[1]), self.imgs.get(words[2])) {
            let widthv1 = self.memory[indexv1].width();
            let heightv1 = self.memory[indexv1].height();
            let widthv2 = self.memory[indexv2].width();
            let heightv2 = self.memory[indexv2].height();
            let cloned_v2_mem = self.memory[indexv2].clone();
            for xi in (std::cmp::max(pos_x, 0) as u32)..(std::cmp::min(widthv1 as i32 , widthv2 as i32 / scale as i32 + pos_x) as u32) {
                for yi in (std::cmp::max(pos_y, 0) as u32)..(std::cmp::min(heightv1 as i32 , heightv2 as i32 / scale as i32+ pos_y) as u32) {
                    let pixelv1 = self.memory[indexv1].get_pixel_mut(xi as u32, yi as u32);
                    let pixelv2 = cloned_v2_mem.get_pixel((xi as i32 - pos_x) as u32 * scale, (yi as i32 - pos_y) as u32 * scale);
                    pixelv1.0[0] = ((u32::from(pixelv1.0[0]) * (100 - opacity))/100 + (u32::from(pixelv2.0[0]) * opacity) / 100) as u8;
                    pixelv1.0[1] = ((u32::from(pixelv1.0[1]) * (100 - opacity))/100 + (u32::from(pixelv2.0[1]) * opacity) / 100) as u8;
                    pixelv1.0[2] = ((u32::from(pixelv1.0[2]) * (100 - opacity))/100 + (u32::from(pixelv2.0[2]) * opacity) / 100) as u8;
                }
            }
        } else {
            eprintln!("[ERROR] {} is not a variable", words[1]);
            std::process::exit(5);
        }
    }

    pub fn circle_command(self: &mut Self, words: Vec<&str>) {
        if words.len() < 10 {
            eprintln!("[ERROR] Not enough params for circle command");
            std::process::exit(3);
        }
        
        if prelude::word_type(words[1].to_string()) != prelude::WordType::Variable 
        || prelude::word_type(words[2].to_string()) != prelude::WordType::NumValue 
        || prelude::word_type(words[3].to_string()) != prelude::WordType::NumValue
        || prelude::word_type(words[4].to_string()) != prelude::WordType::NumValue
        || prelude::word_type(words[6].to_string()) != prelude::WordType::NumValue
        || prelude::word_type(words[7].to_string()) != prelude::WordType::NumValue
        || prelude::word_type(words[8].to_string()) != prelude::WordType::NumValue
        || prelude::word_type(words[9].to_string()) != prelude::WordType::NumValue
        {
            eprintln!("[ERROR] Wrong types for params in img command");
            std::process::exit(3);
        }
        
        let pos_x = words[2].to_string().parse::<u32>().unwrap();
        let pos_y = words[3].to_string().parse::<u32>().unwrap();
        let radius = words[4].to_string().parse::<u32>().unwrap();
        let girth = words[5].to_string().parse::<u32>().unwrap();
        let cr = words[6].to_string().parse::<u8>().unwrap();
        let cg = words[7].to_string().parse::<u8>().unwrap();
        let cb = words[8].to_string().parse::<u8>().unwrap();
        let opacity = words[9].to_string().parse::<u32>().unwrap();

        if let Some(&indexv1) = self.imgs.get(words[1]) {
            let widthv1 = self.memory[indexv1].width();
            let heightv1 = self.memory[indexv1].height();
            for xi in 0..widthv1 {
                for yi in 0..heightv1 {
                    let dist = ((xi - pos_x).pow(2) + (yi - pos_y).pow(2)).isqrt();
                    if dist >= radius - girth/2 && dist <= radius + girth/2 {
                        let pixelv1 = self.memory[indexv1].get_pixel_mut(xi, yi);
                        pixelv1.0[0] = ((u32::from(pixelv1.0[0]) * (100 - opacity))/100 + (u32::from(cr) * opacity) / 100) as u8;
                        pixelv1.0[1] = ((u32::from(pixelv1.0[1]) * (100 - opacity))/100 + (u32::from(cg) * opacity) / 100) as u8;
                        pixelv1.0[2] = ((u32::from(pixelv1.0[2]) * (100 - opacity))/100 + (u32::from(cb) * opacity) / 100) as u8;
                    }
                }
            }
        } else {
            eprintln!("[ERROR] {} is not a variable", words[1]);
            std::process::exit(5);
        }
    }

    pub fn echo_command(self: &Self, words: Vec<&str>) {
        for word in &words[1..] {
            print!("{word} ");
        }
    }

    pub fn canvas_command(self: &Self, words: Vec<&str>) {
        println!("canvas {:?}", words);
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
            prelude::WordType::CanvasCommand => {
                self.canvas_command(words);
            },
            prelude::WordType::CircleCommand => {
                self.circle_command(words);
            },
            prelude::WordType::DownscaleCommand => {
                self.downscale_command(words);
            },
            prelude::WordType::EchoCommand => {
                self.echo_command(words);
            },
            prelude::WordType::Variable => {
                println!(" ");
            },
            _ => {
                println!(" ");
            }
        }
    }

    pub fn start(self: &mut Self) {
        while let Some(line) = self.next_line() {
            if ! line.starts_with("#") {
                self.interpret(line);
            }
        }   
    }
}


