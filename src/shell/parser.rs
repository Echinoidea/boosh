use std::process::{Command, Stdio};

pub trait Expandable {
    /// Expand commands like $(command)
    fn expand_sub_command(self: &Self, input: &str) -> String {
        let mut result = String::new();
        let mut chars = input.chars().peekable();

        while let Some(c) = chars.next() {
            // Check for $(
            if c == '$' && chars.peek() == Some(&'(') {
                chars.next(); // Consume the ( and continue

                let mut command = String::new();
                let mut paren_count = 1; // For nesting

                while let Some(inner_c) = chars.next() {
                    // Find further open parenthesis, if closing,
                    // decrement paren_count, if no more paren, break
                    if inner_c == '(' {
                        paren_count += 1;
                    } else if inner_c == ')' {
                        paren_count -= 1;
                        if paren_count == 0 {
                            break;
                        }
                    }

                    command.push(inner_c);
                }

                let output = self.execute_sub_command(&command);
                result.push_str(&output);
            } else if c == '$' && chars.peek().is_some() {
                // Handle variables
                let mut var_name = String::new();
                while let Some(&next_c) = chars.peek() {
                    if next_c.is_alphanumeric() || next_c == '_' {
                        var_name.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }

                if !var_name.is_empty() {
                    let value = self.expand_variable(&var_name);
                    result.push_str(&value);
                } else {
                    result.push(c);
                }
            } else {
                result.push(c);
            }
        }

        result
    }

    // Todo, should be Option<String>, handle properly upstream
    fn execute_sub_command(self: &Self, command: &str) -> String {
        let tokens: Vec<&str> = command.split_whitespace().collect();
        if tokens.is_empty() {
            return String::new();
        }

        let (program, args) = tokens.split_first().unwrap();

        match Command::new(program)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
        {
            Ok(output) => String::from_utf8_lossy(&output.stdout)
                .trim_end()
                .to_string(),
            Err(_) => String::new(),
        }
    }

    fn expand_variable(&self, var_name: &str) -> String {
        match var_name {
            "USER" => std::env::var("USER").unwrap_or_else(|_| "unknown".to_string()),
            "PWD" => std::env::current_dir()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|_| "unknown".to_string()),
            "HOME" => std::env::var("HOME").unwrap_or_else(|_| "unknown".to_string()),
            _ => {
                // get from environment
                std::env::var(var_name).unwrap_or_else(|_| format!("${}", var_name))
            }
        }
    }
}

pub trait Tokenizable: Expandable {
    fn parse_tokens(self: &Self, input: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut current_token = String::new();
        let mut chars = input.chars().peekable();
        let mut in_quotes = false;
        let mut quote_char = '"';

        while let Some(c) = chars.next() {
            match c {
                '"' | '\'' if !in_quotes => {
                    in_quotes = true;
                    quote_char = c;
                }
                '"' | '\'' if in_quotes && c == quote_char => {
                    in_quotes = false;

                    if !current_token.is_empty() {
                        let expanded = self.expand_sub_command(&current_token);
                        tokens.push(expanded);
                        current_token.clear();
                    }
                }
                ' ' | '\t' | '\n' if !in_quotes => {
                    if !current_token.is_empty() {
                        let expanded = self.expand_sub_command(&current_token);
                        tokens.push(expanded);
                        current_token.clear();
                    }
                }
                _ => {
                    current_token.push(c);
                }
            }
        }

        // Last token
        if !current_token.is_empty() {
            let expanded = self.expand_sub_command(&current_token);
            tokens.push(expanded);
        }

        tokens
    }
}
