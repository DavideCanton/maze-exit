custom_derive! {
    #[derive(Debug, EnumFromStr)]
    pub enum UIType {
        Terminal,
        Gui,
        Noop,
    }
}
