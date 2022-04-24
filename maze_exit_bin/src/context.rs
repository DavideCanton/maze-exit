use std::error::Error;

use show_image::run_context;

use crate::app::app_enums::UIType;

pub type ContextResult = Result<(), Box<dyn Error>>;

pub trait Context<F> {
    fn run(&self, f: F) -> ContextResult;
}

struct PlainContext;

impl<F> Context<F> for PlainContext
where
    F: FnOnce() -> ContextResult,
{
    fn run(&self, f: F) -> ContextResult {
        f()
    }
}

struct ShowImageContext;

impl<F> Context<F> for ShowImageContext
where
    F: FnOnce() -> ContextResult + Send + 'static,
{
    fn run(&self, f: F) -> ContextResult {
        run_context(f)
    }
}

pub fn create_context<F>(ui_type: UIType) -> Result<Box<dyn Context<F>>, Box<dyn Error>>
where
    F: FnOnce() -> ContextResult + Send + 'static,
{
    let context: Box<dyn Context<F>> = match ui_type {
        UIType::Gui => Box::new(ShowImageContext),
        _ => Box::new(PlainContext),
    };

    Ok(context)
}
