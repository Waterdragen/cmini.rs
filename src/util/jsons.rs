use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use serde_json::Value;
use crate::util::analyzer_util::ThreeFingerCombo;
use crate::util::core::Layout;

fn read_json(path: &str) -> Value {
    let mut file = File::open(path).expect(
        format!("Failed to open file {}", path).as_str()
    );
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");
    let json: Value = serde_json::from_str(&contents).expect("Failed to parse JSON");
    json
}

pub fn get_map_str_str(path: &str) -> HashMap<String, String> {
    let json = read_json(path);
    let mut hashmap: HashMap<String, String> = HashMap::new();

    if let Value::Object(obj) = json {
        for (key, value) in obj {
            if let Value::String(str_) = value {
                hashmap.insert(key, str_);
            }
        }
    }
    hashmap
}

pub fn get_vec_str(path: &str) -> Vec<String> {
    let json = read_json(path);
    match json {
        Value::Array(arr) => {
            arr.into_iter()
                .filter_map(|val| Some(val.to_string()))
                .collect()
        }
        _ => Vec::new()
    }
}

pub fn get_map_str_vec_u64(path: &str) -> HashMap<String, Vec<u64>> {
    let json = read_json(path);
    let mut hashmap: HashMap<String, Vec<u64>> = HashMap::new();

    if let Value::Object(obj) = json {
        for (key, value) in obj {
            if let Value::Array(arr) = value {
                let vec_ = arr
                    .into_iter()
                    .filter_map(|val| val.as_u64())
                    .collect();
                hashmap.insert(key, vec_);
            }
        }
    }
    hashmap
}

pub fn get_map_str_u64(path: &str) -> HashMap<String, u64> {
    let json = read_json(path);
    let mut hashmap: HashMap<String, u64> = HashMap::new();

    if let Value::Object(obj) = json {
        for (key, value) in obj {
            if let Some(u64_) = value.as_u64() {
                hashmap.insert(key, u64_);
            }
        }
    }
    hashmap
}

pub fn get_layout(path: &str) -> Layout {
    let json = read_json(path);
    serde_json::from_value(json).expect(&format!("Failed to parse layout {}", path))
}

pub fn get_map_u64_vec_str(path: &str) -> HashMap<u64, Vec<String>> {
    let json = read_json(path);
    let mut map: HashMap<u64, Vec<String>> = HashMap::new();
    if let Value::Object(obj) = json {
        for (id_str, names) in obj {
            match id_str.parse::<u64>() {
                Err(_) => continue,
                Ok(u64_) => {
                    if let Value::Array(arr) = names {
                        let arr: Vec<String> = arr.iter()
                                                  .filter_map(|v| v.as_str())
                                                  .map(|s| s.to_string())
                                                  .collect();
                        map.insert(u64_, arr);
                    }
                },
            }
        }
    }
    map
}

pub fn get_vec_vec_char_f64(path: &str) -> Vec<(Vec<char>, f64)> {
    let json = read_json(path);
    if let Value::Object(obj) = json {
        obj.into_iter()
            .flat_map(|(key, value)| {
                let mut chars: Vec<char> = Vec::with_capacity(3);
                key.to_lowercase().chars().for_each(|c| {
                    chars.push(c);
                });
                let number: f64 = value.as_f64().unwrap_or(0.0);
                Some((chars, number))
            })
            .collect()
    }
    else {
        Vec::new()
    }
}

pub fn get_map_str_map_str_f64(path: &str) -> HashMap<String, HashMap<String, f64>> {
    let json = read_json(path);
    let mut map: HashMap<String, HashMap<String, f64>> = HashMap::new();
    if let Value::Object(obj) = json {
        for (corpus, stat) in obj {
            if let Value::Object(stat) = stat {
                let stat_map: HashMap<String, f64>
                    = stat.iter()
                          .filter_map(|item| {
                        match item.1.as_f64() {
                            Some(f64_) => Some((item.0.to_string(), f64_)),
                            None => None,
                        }
                    }).collect();
                map.insert(corpus, stat_map);
            }
        }
    }
    map
}

pub fn get_table(table_path: &str) -> HashMap<ThreeFingerCombo, String> {
    let json = read_json(table_path);
    let mut hashmap: HashMap<ThreeFingerCombo, String> = HashMap::new();

    if let Value::Object(obj) = json {
        for (key, value) in obj {
            if let Value::String(str_) = value {
                hashmap.insert(ThreeFingerCombo::new(key), str_);
            }
        }
    }
    hashmap
}

pub fn write_map_str_map_str_f64(path: &str, map: &HashMap<String, HashMap<String, f64>>) {
    let file = File::create(path).unwrap();
    serde_json::to_writer_pretty(file, map).unwrap();
}

pub fn write_layout(path: &str, ll: &Layout) {
    let file = File::create(path).unwrap();
    serde_json::to_writer_pretty(file, ll).unwrap();
}

pub fn write_map_u64_vec_str(path: &str, map: &HashMap<u64, Vec<String>>) {
    let file = File::create(path).unwrap();
    serde_json::to_writer_pretty(file, map).unwrap();
}

#[cfg(test)]
mod tests {
    use std::hash::Hash;
    use super::*;

    fn test_get_map_str_str() {
        let path = "./links.json";
        let map = get_map_str_str(path);
        for (key, value) in map.into_iter() {
            println!("{}, {}", key, value);
        }
    }

    fn test_get_vec_str() {
        let path = "./pairs.json";
        let vec = get_vec_str(path);
        for value in vec.into_iter() {
            println!("{}", value);
        }
    }

    fn test_get_map_str_vec_u64() {
        let path = "./likes.json";
        let map = get_map_str_vec_u64(path);
        for (key, vec_) in map.into_iter() {
            println!("{}, {:?}", key, vec_);
        }
    }

    fn test_get_map_str_u64() {
        let path = "./authors.json";
        let map = get_map_str_u64(path);
        for (key, value) in map.into_iter() {
            println!("{}, {}", key, value);
        }
    }

    fn test_get_layout() {
        let path = "./layouts/a02.json";
        let layout = get_layout(path);
        println!("{:?}", layout);
    }

    fn test_write_layout() {
        let layout = Layout {
            name: String::from("bogos"),
            user: 12345,
            board: String::from("binted"),
            keys: HashMap::new(),
            free: Vec::new(),
        };
        write_layout("./bogos_binted.json", &layout);
    }

    fn test_get_map_u64_vec_str() {
        let path = "./authors.json";
        let map = get_map_u64_vec_str(path);
        for (key, value) in map {
            println!("{key}: {:?}", value);
        }
    }

    fn test_get_map_str_map_str_f64() {
        let path = "./cache/a02.json";
        let map = get_map_str_map_str_f64(path);
        for (key, value) in map {
            println!("{key}: {:?}", value);
        }
    }

    fn test_get_vec_vec_char_f64() {
        let path = "./corpora/english-1k/trigrams.json";
        let vec_ = get_vec_vec_char_f64(path);
        println!("{:?}", vec_);
    }
}
