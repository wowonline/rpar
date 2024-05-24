use std::process;

use rayon::prelude::*;

pub struct Executor {
    times: usize,
    silent: bool,
    parallel: bool,
    command: String,
    command_args: Vec<String>,
}

impl Executor {
    pub fn new(
        times: usize,
        silent: bool,
        parallel: bool,
        command: String,
        command_args: Vec<String>,
    ) -> Self {
        Executor {
            times: times,
            silent: silent,
            parallel: parallel,
            command: command,
            command_args: command_args,
        }
    }

    pub fn execute(&self) {
        if self.parallel {
            (0..self.times).into_par_iter().for_each(|_| self.exec());
        } else {
            for _ in 0..self.times {
                self.exec();
            }
        }
    }

    fn exec(&self) {
        let output = process::Command::new(&self.command)
            .args(&self.command_args)
            .output()
            .expect(&format!(
                "failed to execute {:?} command with args: {:?}",
                self.command, self.command_args
            ));
        if !self.silent {
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
    }
}
