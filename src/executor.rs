use std::{process, time::{Duration, Instant}};

use rayon::prelude::*;

pub struct Executor {
    times: usize,
    silent: bool,
    parallel: bool,
    to_benchmark: bool,
    command: String,
    command_args: Vec<String>,
}

impl Executor {
    pub fn new(
        times: usize,
        silent: bool,
        parallel: bool,
        to_benchmark: bool,
        command: String,
        command_args: Vec<String>,
    ) -> Self {
        Executor {
            times: times,
            silent: silent,
            parallel: parallel,
            to_benchmark: to_benchmark,
            command: command,
            command_args: command_args,
        }
    }

    pub fn execute(&self) {
        if self.to_benchmark {
            let now = Instant::now();
            self.exec();
            let elapsed_time = now.elapsed();
            println!("Elapsed time: {}ms", elapsed_time.as_millis());
        } else {
            self.exec();
        }
    }

    fn exec(&self) {
        if self.parallel {
            (0..self.times).into_par_iter().for_each(|_| self.create_process_and_execute());
        } else {
            for _ in 0..self.times {
                self.create_process_and_execute();
            }
        }
    }

    fn create_process_and_execute(&self) {
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
