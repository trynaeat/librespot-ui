use std::{ops::DerefMut, process::{Child, Command}};

pub struct LibrespotInst {
    process: Option<Child>
}

#[derive(Debug)]
pub struct InstExistsError {}
#[derive(Debug)]
pub struct InstDoesNotExistError {}

impl LibrespotInst {
    pub fn new() -> Self {
        Self { process: None }
    }

    pub fn spawn_librespot(&mut self) -> Result<(), InstExistsError>{
        if(self.process.is_none()) {
            let result = Command::new("librespot")
                .arg("--backend")
                .arg("pipe")
                .spawn()
                .unwrap();
            self.process = Some(result);
            return Ok(())
        } else {
            Err(InstExistsError {})
        }
    }

    pub fn kill_librespot(&mut self) -> Result<(), InstDoesNotExistError> {
        let result = match &mut self.process {
            Some(ref mut p) => p.kill().map_err(|_| InstDoesNotExistError {}),
            None => Err(InstDoesNotExistError {}),
        };
        self.process = None;
        return result;
    }
}