
#[derive(PartialEq)]
pub enum WordType {
    ImgCommand,
    SaveCommand,
    LayerCommand,
    CanvasCommand,
    CircleCommand,
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

    if word == statics::JPG_EXTENSION.to_string() {
        return WordType::Extension;
    }

    if word.starts_with("'") && word.ends_with("'") {
        return WordType::StringValue;
    }

    if word.chars().all(|c| c.is_numeric()) {
        return WordType::NumValue;
    }

    WordType::Variable
}
