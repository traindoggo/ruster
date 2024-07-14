#[derive(Debug, PartialEq)]
pub enum ArgOption {
    Help,
    Show,
    Add,
    Remove,
    Edit,
    Done,
}

/// Parse a user input
///
/// (no args) : show all todos
/// help      : show help
/// add       : add todos
/// remove    : remove todos
/// edit      : edit todos
/// done      : done todos
///
/// # Examples
///
/// help
/// add    <name> <name> ...
/// remove <usize> <usize> ...
/// edit   <usize>
/// done   <usize> <usize> ...
#[derive(Debug, PartialEq)]
pub struct Config {
    pub option: ArgOption,
    pub args: Vec<String>,
}

impl Config {
    pub fn new(args: Vec<String>) -> Result<Config, &'static str> {
        let (option, args) = if args.len() == 1 {
            (ArgOption::Show, vec![])
        } else {
            match &*args[1].to_string() {
                "help" => (ArgOption::Help, vec![]),
                "add" => (ArgOption::Add, args[2..].to_vec()),
                "remove" => (ArgOption::Remove, args[2..].to_vec()),
                "edit" => (ArgOption::Edit, args[2..].to_vec()),
                "done" => (ArgOption::Done, args[2..].to_vec()),
                _ => return Err("<Config>::Wrong Option"),
            }
        };

        // TODO: better rustacean way
        match option {
            ArgOption::Help => {}

            ArgOption::Show => {
                // never reached?
                if args.len() > 0 {
                    return Err("<Config>::Show too much args");
                }
            }

            ArgOption::Add => {
                if args.len() <= 0 {
                    return Err("<Config>::Add not enough args");
                }
            }

            ArgOption::Remove => {
                if args.len() < 1 {
                    return Err("<Config>::Remove not enough args");
                } else if args.len() > 1 {
                    return Err("<Config>::Remove too much args");
                }

                if let Err(_) = (&args[0]).parse::<usize>() {
                    return Err("<Config>::Remove wrong type of args");
                }
            }

            ArgOption::Edit => {
                if args.len() < 2 {
                    return Err("<Config>::Edit not enough args");
                }
                if args.len() > 2 {
                    return Err("<Config>::Edit too much args");
                }

                if let Err(_) = (&args[0]).parse::<usize>() {
                    return Err("<Config>::Edit wrong type of args");
                }
            }

            ArgOption::Done => {
                if args.len() < 1 {
                    return Err("<Config>::Done not enough args");
                }

                for arg in &args {
                    if let Err(_) = arg.parse::<usize>() {
                        return Err("<Config>::Done wrong type of args");
                    }
                }
            }
        }

        Ok(Config { option, args })
    }
}

#[cfg(test)]
mod config_test {
    use rstest::*;

    use super::*;

    fn create_args(envs: Vec<&str>) -> Vec<String> {
        return envs.iter().map(|s| s.to_string()).collect();
    }

    #[rstest]
    #[case(create_args(vec!["target/debug/tester", "hoge"]), "<Config>::Wrong Option")]
    fn wrong_option(#[case] args: Vec<String>, #[case] msg: &str) {
        let ret = Config::new(args);
        assert_eq!(ret, Err(msg));
    }

    #[rstest]
    #[case(create_args(vec!["target/debug/tester", "add"]), "<Config>::Add not enough args")]
    #[case(create_args(vec!["target/debug/tester", "edit"]), "<Config>::Edit not enough args")]
    #[case(create_args(vec!["target/debug/tester", "remove"]), "<Config>::Remove not enough args")]
    #[case(create_args(vec!["target/debug/tester", "done"]), "<Config>::Done not enough args")]
    fn not_enough_args(#[case] args: Vec<String>, #[case] msg: &str) {
        assert_eq!(Config::new(args), Err(msg));
    }

    #[rstest]
    #[case(create_args(vec!["target/debug/tester", "edit", "1", "2", "3"]), "<Config>::Edit too much args")]
    #[case(create_args(vec!["target/debug/tester", "edit", "1", "2", "hoge"]), "<Config>::Edit too much args")]
    #[case(create_args(vec!["target/debug/tester", "remove", "2", "3", "4"]), "<Config>::Remove too much args")]
    #[case(create_args(vec!["target/debug/tester", "remove", "1", "hoge"]), "<Config>::Remove too much args")]
    fn too_much_args(#[case] args: Vec<String>, #[case] msg: &str) {
        assert_eq!(Config::new(args), Err(msg));
    }

    #[rstest]
    #[case(create_args(vec!["target/debug/tester", "edit", "hoge", "hoge"]), "<Config>::Edit wrong type of args")]
    #[case(create_args(vec!["target/debug/tester", "edit", "hoge", "1"]), "<Config>::Edit wrong type of args")]
    #[case(create_args(vec!["target/debug/tester", "remove", "hage"]), "<Config>::Remove wrong type of args")]
    #[case(create_args(vec!["target/debug/tester", "done", "hige"]), "<Config>::Done wrong type of args")]
    #[case(create_args(vec!["target/debug/tester", "done", "1", "hige"]), "<Config>::Done wrong type of args")]
    fn wrong_type_of_args(#[case] args: Vec<String>, #[case] msg: &str) {
        assert_eq!(Config::new(args), Err(msg));
    }
}
