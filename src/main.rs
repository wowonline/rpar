mod cli;
mod executor;

use std::env;

// usage of command:
// rpar [RPAR ARGUMENTS] -- COMMAND [COMMAND ARGUMENTS]

// Examples of usage:
//
// 1)
//      ./rpar -t 10 -s -- ls -l
//                   or
//      cargo run -- -t 10 -s -- ls -l
// 2)
//      ./rpar -t 1 -u -- curl www.google.com

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
//
// arg  --to_benchmark     /  -b  - to measure time from start to end of executor's work.
//                                  Default: time is not measured

fn main() {
    let args: Vec<String> = env::args().collect();

    let cli = cli::Cli::parse_args(args);

    let (times, silent, parallel, to_benchmark, command, command_args) = cli.get_args();

    let executor = executor::Executor::new(
        times,
        silent,
        parallel,
        to_benchmark,
        command.to_string(),
        command_args.to_vec(),
    );

    executor.execute();
}
