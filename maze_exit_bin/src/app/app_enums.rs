use clap::ValueEnum;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum UIType {
    Terminal,
    #[cfg(feature = "gui")]
    Gui,
    No,
}
