
    pub(crate) struct TokensLines {
        lines: Vec<TokensLine>,
    }

    impl TokensLines {
        pub(crate) fn from_string(string: String, delimiters: Vec<char>) -> Self {
            TokensLines {
                lines: string.lines().map(|line|
                    TokensLine::new(line.to_string(), delimiters.clone())
                ).collect(),
            }
        }

        pub(crate) fn to_vec(&self) -> Vec<Vec<String>> {
            self.lines.iter().map(|line| line.to_vec()).collect()
        }

        pub(crate) fn merge_within_delimiters(&mut self, delimiter: char, dissolve_delimiters: bool) {
            self.lines.iter_mut().map(|line| line.merge_within_delimiters(delimiter, dissolve_delimiters)).collect()
        }
    }

    struct TokensLine {
        line: Vec<String>,
    }

    impl TokensLine {
        fn new(string: String, delimiters: Vec<char>) -> Self {
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
                line: tokens.iter().filter(|token| !token.is_empty()).cloned().collect()
            }
        }

        fn to_vec(&self) -> Vec<String> {
            self.line.clone()
        }

        fn merge_within_delimiters(&mut self, delimiter: char, dissolve_delimiters: bool) {
            let mut new_line: Vec<String> = Vec::new();

            let mut token_built = String::new();
            let mut is_merging_active = false;
            let mut is_merging_state_just_changed ;

            for token in self.line.clone() {

                is_merging_state_just_changed = false;

                if token.parse::<char>().is_ok_and(|token| token == delimiter) {
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
                        token_built.push_str(token.as_str());
                    }

                } else {
                    new_line.push(token.to_string());
                }
            }

            self.line = new_line;
        }
    }
