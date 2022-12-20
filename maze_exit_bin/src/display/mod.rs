use std::error::Error;

use crate::app::app_enums::UIType;

use self::{
    display_trait::Displayer, gui_displayer::GuiDisplayer, noop_displayer::NoopDisplayer,
    term_displayer::TerminalDisplayer,
};

pub mod display_trait;
pub mod gui_displayer;
pub mod noop_displayer;
#[allow(dead_code)]
pub mod term_displayer;

pub fn create_displayer(ui_type: UIType) -> Result<Box<dyn Displayer>, Box<dyn Error>> {
    let displayer: Box<dyn Displayer> = match ui_type {
        UIType::Gui => Box::new(GuiDisplayer::new()?),
        UIType::Terminal => Box::<TerminalDisplayer>::default(),
        UIType::Noop => Box::new(NoopDisplayer),
    };

    Ok(displayer)
}
