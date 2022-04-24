use std::{any::Any, error::Error};

use show_image::run_context;

use crate::app::app_enums::UIType;

pub type ContextResult = Result<(), Box<dyn Error>>;

pub trait Context<F>: Any
where
    F: FnOnce() -> ContextResult,
{
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

#[cfg(test)]
mod tests {
    use std::any::TypeId;
    use test_case::test_case;

    use super::*;

    #[test]
    #[cfg(feature = "gui_test")]
    fn test_create_context_gui() {
        common_ok_test(UIType::Gui, TypeId::of::<ShowImageContext>());
    }

    #[test_case(UIType::Noop; "when ui_type is Noop")]
    #[test_case(UIType::Terminal; "when ui_type is Terminal")]
    fn test_create_context_plain(ui_type: UIType) {
        common_ok_test(ui_type, TypeId::of::<PlainContext>());
    }

    #[test_case(UIType::Noop; "when ui_type is Noop")]
    #[test_case(UIType::Terminal; "when ui_type is Terminal")]
    fn test_run_context_err(ui_type: UIType) {
        common_error_test(ui_type);
    }

    #[test]
    #[cfg(feature = "gui_test")]
    fn test_run_context_gui_err() {
        common_error_test(UIType::Gui);
    }

    fn common_ok_test(ui_type: UIType, type_id: TypeId) {
        let context = create_context(ui_type).unwrap();
        assert_eq!((*context).type_id(), type_id);
        let res = context.run(|| Ok(()));
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), ());
    }

    fn common_error_test(ui_type: UIType) {
        let context = create_context(ui_type).unwrap();
        let res = context.run(|| {
            let err = "Error".to_string().into();
            Err(err)
        });
        assert!(res.is_err());
        assert_eq!(format!("{}", res.err().unwrap()), "Error");
    }
}
