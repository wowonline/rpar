pub struct Cli {
    pub rpar_times: usize,
    pub rpar_silent: bool,
    pub parallel: bool,
    pub to_benchmark: bool,
    pub command: String,
    pub command_args: Vec<String>,
}

impl Cli {
    pub fn get_args(&self) -> (usize, bool, bool, bool, &String, &Vec<String>) {
        (
            self.rpar_times,
            self.rpar_silent,
            self.parallel,
            self.to_benchmark,
            &self.command,
            &self.command_args,
        )
    }

    pub fn parse_args(args: Vec<String>) -> Cli {
        let mut cli = Cli {
            rpar_times: 1,
            rpar_silent: false,
            parallel: true,
            to_benchmark: false,
            command: String::new(),
            command_args: vec![],
        };
        let idx = args.iter().position(|arg| arg == "--").expect(
            "Can't find '--' between rpar and command arguments. Consider looking at 'rpar -h.'",
        );
        cli.parse_rpar_args(args[..idx].to_vec());
        cli.parse_command_args(args[idx + 1..].to_vec());
        cli
    }

    fn parse_rpar_args(&mut self, args: Vec<String>) {
        for (i, arg) in args.iter().enumerate() {
            if arg == "-s" || arg == "--silent" {
                self.rpar_silent = true;
                println!("silent!");
            }
            if arg == "-t" || arg == "--times" {
                self.rpar_times = args[i + 1].parse().unwrap();
                println!("{} times!", self.rpar_times);
            }
            if arg == "-u" || arg == "--unparallel" {
                self.parallel = false;
                println!("unparallel!");
            }
            if arg == "-b" || arg == "--to_benchmark" {
                self.to_benchmark = true;
                println!("to_benchmark!");
            }
        }
    }

    fn parse_command_args(&mut self, args: Vec<String>) {
        self.command.clone_from(&args[0]);
        self.command_args = args[1..].to_vec();
    }
}
