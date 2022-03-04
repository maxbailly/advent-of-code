use std::fmt::{Debug, Formatter, Result};

use crate::Input;

pub struct Wire {
    id: &'static str,
    input: Option<Box<dyn Input>>,
    output: Option<u16>,
}

impl Wire {
    #[inline]
    pub fn new(id: &'static str) -> Self {
        Self {
            id,
            input: None,
            output: None,
        }
    }

    #[inline]
    pub fn id(&self) -> &'static str {
        self.id
    }

    #[inline]
    pub fn plug(&mut self, input: Box<dyn Input>) {
        self.input = Some(input)
    }

    pub fn output(&mut self) -> Option<u16> {
        if self.output.is_none() {
            if let Some(input) = &self.input {
                self.output = input.output()
            }
        }

        self.output
    }

    #[inline]
    pub fn reset(&mut self) {
        self.output = None;
    }
}

impl Debug for Wire {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let input_str = match &self.input {
            Some(input) => input.dbg_string(),
            None => String::from("None"),
        };

        write!(
            f,
            "Wire {{ id: {}, input: {:?}, output: {:?}}}",
            self.id, input_str, self.output
        )
    }
}
