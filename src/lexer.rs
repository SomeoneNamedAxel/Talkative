pub(crate) struct TokensLines {
    lines: Vec<TokensLine>,
}

impl TokensLines {
    pub(crate) fn from_string(string: String, delimiters: Vec<char>, keywords: Vec<String>, operators: Vec<String>) -> Self {
        TokensLines {
            lines: string
                .lines()
                .map(|line| TokensLine::new(line.to_string(), &delimiters, &keywords, &operators))
                .collect(),
        }
    }

    pub(crate) fn to_vec(&self) -> Vec<Vec<Token>> {
        self.lines.iter().map(|line| line.to_vec()).collect()
    }

    pub(crate) fn to_string_vec(&self) -> Vec<Vec<String>> {
        self.lines.iter().map(|line| line.to_string_vec()).collect()
    }

    pub(crate) fn merge_within_delimiters(&mut self, delimiter: char, dissolve_delimiters: bool) {
        self.lines
            .iter_mut()
            .map(|line| line.merge_within_delimiters(delimiter))
            .collect()
    }
}

struct TokensLine {
    line: Vec<Token>,
}

impl TokensLine {
    fn new(string: String, delimiters: &Vec<char>, keywords: &Vec<String>, operators: &Vec<String>) -> Self {
        let mut tokens: Vec<String> = Vec::new();
        let mut current_token = String::new();

        for character in string.chars() {
            if delimiters.contains(&character) {
                tokens.push(current_token.clone());
                current_token.clear();

                tokens.push(character.to_string());
            } else {
                current_token.push(character);
            }
        }
        tokens.push(current_token.clone());

        TokensLine {
            line: tokens
                .into_iter()
                .filter(|token_str| !token_str.is_empty())
                .map(|token_str| Token::new(Self::identify_token(&token_str, &keywords, &operators), token_str))
                .collect(),
        }
    }

    fn identify_token(token: &String, keywords: &Vec<String>, operators: &Vec<String>) -> String {
        if keywords.contains(&token) {
            String::from("keyword")
        } else if operators.contains(&token) {
            String::from("operator")
        } else if *token == " ".to_string() || *token == "\"" {
            String::from("separator")
        } else {
            String::from("literal")
        }
    }

    fn to_vec(&self) -> Vec<Token> {
        self.line.clone()
    }

    fn to_string_vec(&self) -> Vec<String> {
        self.line.iter().map(|line| line.to_string()).collect()
    }

    fn merge_within_delimiters(&mut self, delimiter: char) {
        let mut new_line: Vec<Token> = Vec::new();

        let mut token_built = Token::new("literal".parse().unwrap(), String::new());
        let mut is_merging_active = false;
        let mut is_merging_state_just_changed;

        for token in self.line.clone() {
            is_merging_state_just_changed = false;

            if token
                .value
                .parse::<char>()
                .is_ok_and(|token| token == delimiter)
            {
                is_merging_active = !is_merging_active;

                if !is_merging_active {
                    new_line.push(token_built.clone());
                }

                is_merging_state_just_changed = true;
            }

            if is_merging_active {
                if is_merging_state_just_changed {
                    new_line.push(token);
                } else {
                    token_built =
                        Token::new(token_built.designation, token_built.value + &*token.value);
                }
            } else {
                new_line.push(token);
            }
        }

        self.line = new_line;
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Token {
    designation: String,
    value: String,
}

impl Token {
    fn new(designation: String, value: String) -> Self {
        Token { designation, value }
    }

    fn to_string(&self) -> String {
        format!("({}) {}", self.designation, self.value)
    }
}
