pub trait Input {
    fn output(&self) -> Option<u16>;
    fn dbg_string(&self) -> String;

    fn into_input(self) -> Box<dyn Input>
        where Self: Sized + 'static
    {
        Box::new(self)
    }
}
