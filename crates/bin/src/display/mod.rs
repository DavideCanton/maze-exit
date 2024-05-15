use crate::app::app_enums::UIType;
use anyhow::Result;

use self::{
    display_trait::Displayer, no_displayer::NoDisplayer, term_displayer::TerminalDisplayer,
};

pub mod display_trait;
#[cfg(feature = "gui")]
pub mod gui_displayer;
pub mod no_displayer;
#[allow(dead_code)]
pub mod term_displayer;

pub fn create_displayer(ui_type: UIType) -> Result<Box<dyn Displayer>> {
    let displayer: Box<dyn Displayer> = match ui_type {
        #[cfg(feature = "gui")]
        UIType::Gui => Box::new(gui_displayer::GuiDisplayer::new()?),
        UIType::Terminal => Box::<TerminalDisplayer>::default(),
        UIType::No => Box::new(NoDisplayer),
    };

    Ok(displayer)
}
