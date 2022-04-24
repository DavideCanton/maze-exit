custom_derive! {
    #[derive(Debug, EnumFromStr, Clone, Copy)]
    pub enum UIType {
        Terminal,
        Gui,
        Noop,
    }
}
