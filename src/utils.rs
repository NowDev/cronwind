use std::collections::HashMap;

pub fn expand_env_vars(str_raw: &str, env_vars: &HashMap<String, String>) -> String {
    let chars: Vec<char> = str_raw.chars().collect();
    let mut parse_result = String::new();
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '$' {
            if i + 1 < chars.len() && chars[i + 1] == '{' {
                let mut var_name_idx = i + 2;
                while var_name_idx < chars.len() && chars[var_name_idx] != '}' {
                    var_name_idx += 1;
                }

                if var_name_idx < chars.len() {
                    let var_name: String = chars[i + 2..var_name_idx].iter().collect();
                    let parsed_data = env_vars.get(&var_name).cloned().unwrap_or_default();
                    parse_result.push_str(&parsed_data);
                    i = var_name_idx + 1;
                } else {
                    parse_result.push('$');
                    i += 1;
                }
            } else {
                parse_result.push('$');
                i += 1;
            }
        } else {
            parse_result.push(chars[i]);
            i += 1;
        }
    }

    parse_result
}