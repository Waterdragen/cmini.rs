use std::collections::HashMap;
use crate::util::core::{ArgType, KwargType, KwargValue};

pub fn get_args(s: &str) -> Vec<&str> {
    s.split_whitespace().collect()
}

pub fn get_layout(s: &str) -> (String, String) {
    let parts: Vec<&str> = s.split("```").collect();

    let name = parts.get(0).unwrap_or(&"").trim().to_lowercase();
    let matrix = parts.get(1).unwrap_or(&"").trim().to_lowercase();

    (name, matrix)
}

pub fn get_kwargs(s: &str, arg_type: ArgType, cmd_kwargs: &HashMap<String, KwargType>)
                  -> HashMap<String, KwargValue> {
    let words: Vec<&str> = s.split_whitespace().collect();

    let mut arg_index = 0usize;
    for word in words.iter() {
        if is_kwarg(cmd_kwargs, word) {
            break;
        }
        arg_index += 1;
    }

    // Make default hashmap
    let args: Vec<String> = words[..arg_index].iter().map(|s| s.to_string()).collect();
    let mut parsed_kwargs: HashMap<String, KwargValue> = HashMap::new();
    parsed_kwargs.insert(String::from("args"),
                         if arg_type == ArgType::Str {KwargValue::from_str(args.join(" "))}
                            else {KwargValue::from_vec(args)});
    for (kw_name, _) in cmd_kwargs.iter() {
        parsed_kwargs.insert(String::from(kw_name), KwargValue::from_none());
    }

    let words: &Vec<&str> = &words[arg_index..].to_vec();
    let mut last_in_vec = 0usize;
    let mut last_kwarg_type = &KwargType::Vec;
    let mut last_vec_kwarg = String::new();
    let mut in_vec = false;
    for (index, word) in words.into_iter().enumerate() {
        if !is_kwarg(cmd_kwargs, word) {
            continue;
        }
        let word = remove_kw_prefix(word);
        let kw_type = cmd_kwargs.get(&word).unwrap();

        // Encountered next keyword, stops previous vec
        if in_vec {
            parsed_kwargs.insert(last_vec_kwarg.clone(),
                if last_kwarg_type == &KwargType::Vec
                    {KwargValue::from_vec(words[last_in_vec..index].iter().map(|s| s.to_string()).collect())}
                    else {KwargValue::from_str(words[last_in_vec..index].join(" "))}
            );
        }
        in_vec = kw_type == &KwargType::Vec || kw_type == &KwargType::Str;
        if !in_vec {
            parsed_kwargs.insert(word.clone(), KwargValue::from_bool(true));
        }

        // Starts a new list after kwarg
        if in_vec {
            last_kwarg_type = kw_type;
            last_vec_kwarg = word;
            last_in_vec = index + 1;
        }
    }

    // Close the last vec
    if in_vec {
        parsed_kwargs.insert(last_vec_kwarg,
            if last_kwarg_type == &KwargType::Vec {KwargValue::from_vec(words[last_in_vec..].iter().map(|s| s.to_string()).collect())}
                else {KwargValue::from_str(words[last_in_vec..].join(" "))}
        );
    }

    parsed_kwargs
}

fn starts_with_kw_prefix(word: &str) -> bool {
    vec!["--", "—", "––"].iter().any(|prefix| word.starts_with(prefix))
}

fn remove_kw_prefix(word: &str) -> String {
    let mut word = word.to_string();
    for prefix in vec!["--", "—", "––"].into_iter() {
        if let Some(new_word) = word.strip_prefix(prefix) {
            word = new_word.to_string();
        }
    }
    word.to_lowercase()
}

fn is_kwarg(kwargs: &HashMap<String, KwargType>, word: &str) -> bool {
    if !starts_with_kw_prefix(word) {
        return false;
    }
    let word = remove_kw_prefix(word);
    kwargs.contains_key(&word)
}