use crate::output::Output;

pub struct OutputCollection {
    outputs: Vec<Output>,
}

impl OutputCollection {
    pub fn new() -> OutputCollection {
        OutputCollection {
            outputs: Vec::new(),
        }
    }

    pub fn add(&mut self, output: Output) {
        self.outputs.push(output);
    }

    pub fn enable_all(&mut self, state: bool) -> Result<(), String> {
        for o in self.outputs.as_mut_slice() {
            o.set_enabled(state)?;
        }
        Ok(())
    }

    pub fn update_all(&mut self) -> Result<(), String> {
        for o in self.outputs.as_mut_slice() {
            o.update()?;
        }
        Ok(())
    }
}
