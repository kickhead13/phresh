
#[derive(PartialEq)]
pub enum WordType {
    ImgCommand,
    SaveCommand,
    LayerCommand,
    CanvasCommand,
    CircleCommand,
    DownscaleCommand,
    VFlipCommand,
    HFlipCommand,
    EchoCommand,
    ExitCommand,
    Variable,
    StringValue,
    Extension,
    NumValue
}

use crate::lexer::statics;

pub fn word_type(word: String) -> WordType {
    if word == statics::IMG_COMMAND.to_string() {
        return WordType::ImgCommand;
    }

    if word == statics::LAYER_COMMAND.to_string() {
        return WordType::LayerCommand;
    }

    if word == statics::SAVE_COMMAND.to_string() {
        return WordType::SaveCommand;
    }

    if word == statics::CANVAS_COMMAND.to_string() {
        return WordType::CanvasCommand;
    }

    if word == statics::CIRCLE_COMMAND.to_string() {
        return WordType::CircleCommand;
    }
    
    if word == statics::DOWNSCALE_COMMAND.to_string() {
        return WordType::DownscaleCommand;
    }
    
    if word == statics::VFLIP_COMMAND.to_string() {
        return WordType::VFlipCommand;
    }

    if word == statics::HFLIP_COMMAND.to_string() {
        return WordType::HFlipCommand;
    }

    if word == statics::ECHO_COMMAND.to_string() {
        return WordType::EchoCommand;
    }


    if word == statics::EXIT_COMMAND.to_string() {
        return WordType::ExitCommand;
    }
 
    if word == statics::JPG_EXTENSION.to_string() {
        return WordType::Extension;
    }

    if word.starts_with("'") && word.ends_with("'") {
        return WordType::StringValue;
    }

    if word.chars().all(|c| c.is_numeric()) || (word.starts_with("-") && word[1..].chars().all(|c| c.is_numeric())) {
        return WordType::NumValue;
    }

    WordType::Variable
}
