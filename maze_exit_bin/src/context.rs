use anyhow::Result;
#[cfg(feature = "gui")]
use show_image::run_context;
use std::any::Any;

use crate::app::app_enums::UIType;

pub type ContextResult = Result<()>;

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

#[cfg(feature = "gui")]
struct ShowImageContext;

#[cfg(feature = "gui")]
impl<F> Context<F> for ShowImageContext
where
    F: FnOnce() -> ContextResult + Send + 'static,
{
    fn run(&self, f: F) -> ContextResult {
        run_context(f)
    }
}

pub fn create_context<F>(ui_type: UIType) -> Result<Box<dyn Context<F>>>
where
    F: FnOnce() -> ContextResult + Send + 'static,
{
    let context: Box<dyn Context<F>> = match ui_type {
        #[cfg(feature = "gui")]
        UIType::Gui => Box::new(ShowImageContext),
        _ => Box::new(PlainContext),
    };

    Ok(context)
}

#[cfg(test)]
mod tests {
    use anyhow::bail;
    use std::any::TypeId;
    use test_case::test_case;

    use super::*;

    #[test_case(UIType::No; "when ui_type is Noop")]
    #[test_case(UIType::Terminal; "when ui_type is Terminal")]
    fn test_create_context_plain(ui_type: UIType) {
        common_ok_test(ui_type, TypeId::of::<PlainContext>());
    }

    #[test_case(UIType::No; "when ui_type is Noop")]
    #[test_case(UIType::Terminal; "when ui_type is Terminal")]
    fn test_run_context_err(ui_type: UIType) {
        common_error_test(ui_type);
    }

    // TODO these tests fail
    // #[test]
    // #[cfg(feature = "gui")]
    // fn test_create_context_gui() {
    //     common_ok_test(UIType::Gui, TypeId::of::<ShowImageContext>());
    // }

    // #[test]
    // #[cfg(feature = "gui")]
    // fn test_run_context_gui_err() {
    //     common_error_test(UIType::Gui);
    // }

    fn common_ok_test(ui_type: UIType, type_id: TypeId) {
        let context = create_context(ui_type).unwrap();
        assert_eq!((*context).type_id(), type_id);
        let res = context.run(|| Ok(()));
        assert!(res.is_ok());
    }

    fn common_error_test(ui_type: UIType) {
        let context = create_context(ui_type).unwrap();
        let res = context.run(|| bail!("foo"));
        assert!(res.is_err());
        assert_eq!(format!("{}", res.err().unwrap()), "foo");
    }
}
