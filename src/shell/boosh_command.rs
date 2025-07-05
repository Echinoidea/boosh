/// Struct storing a single command, as in a single program with args. Can be piped.
pub struct BooshCommand<'a> {
    pub program: &'a str,
    pub args: Vec<&'a str>,
}

pub trait Parse {
    /// Construct a new BooshCommand from a raw string input
    fn from_input(input: &String) -> BooshCommand;
}

impl Parse for BooshCommand<'_> {
    fn from_input(input: &String) -> BooshCommand {
        let tokens: Vec<&str> = input.split_whitespace().collect();

        let (program, args) = match tokens.split_first() {
            Some((&first, rest)) => (first, rest.to_vec()),
            None => (":", Vec::new()),
        };

        BooshCommand { program, args }
    }
}
