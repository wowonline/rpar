mod cli;

use std::env;
use std::process;

use rayon::prelude::*;

// usage of command:
// rpar [RPAR ARGUMENTS] -- COMMAND [COMMAND ARGUMENTS]

// Examples of usage:
//
// 1)
//      ./rpar -t 10 -s -- ls -l
//                   or
//      cargo run -- -t 10 -s -- ls -l
// 2)
//      ./rpar -t

// rpar arguments:
//
// arg  --times [TIMES] /  -t     - to execute command [TIMES] times
//                                  Default: command is executed 1 time
//
// arg  --silent        /  -s     - to suppress output.
//                                  Default: output is not suppressed
//
// arg  --unparallel    /  -u     - to execute command in sequential way.
//                                  Default: command is executed in a parallel way

fn exec(silent: bool, command: &String, arguments: &Vec<String>) {
    let output = process::Command::new(command)
        .args(arguments)
        .output()
        .expect(&format!(
            "failed to execute {:?} command with args: {:?}",
            command, arguments
        ));
    if !silent {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut binding = cli::Cli::new();
    let cli = binding.parse_args(args);

    let command = &cli.command;
    let arguments = &cli.command_args;
    let silent = cli.rpar_silent;
    let times = cli.rpar_times;

    if cli.parallel {
        (0..times)
            .into_par_iter()
            .for_each(|_| exec(silent, command, arguments));
    } else {
        for _ in 0..times {
            exec(silent, command, arguments);
        }
    }
}
