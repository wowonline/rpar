pub struct Cli {
    pub rpar_times: usize,
    pub rpar_silent: bool,
    pub parallel: bool,
    pub command: String,
    pub command_args: Vec<String>,
}

impl Cli {
    pub fn new() -> Self {
        Cli {
            rpar_times: 1,
            rpar_silent: false,
            parallel: true,
            command: String::new(),
            command_args: vec![],
        }
    }

    pub fn parse_args(&mut self, args: Vec<String>) -> &mut Self {
        let idx = args.iter().position(|arg| arg == "--").expect(
            "Can't find '--' between rpar and command arguments. Consider looking at 'rpar -h.'",
        );
        self.parse_rpar_args(args[..idx].to_vec());
        self.parse_command_args(args[idx + 1..].to_vec());
        self
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
        }
    }

    fn parse_command_args(&mut self, args: Vec<String>) {
        self.command.clone_from(&args[0]);
        self.command_args = args[1..].to_vec();
    }
}
