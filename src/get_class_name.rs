use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn get_class_id_to_class_name() -> io::Result<HashMap<String, String>> {
    let file = File::open("./src/syllabus.txt")?;
    let reader = BufReader::new(file);

    let mut hash_map = HashMap::new();

    for line in reader.lines() {
        let line = line?; 
        let parts: Vec<&str> = line.split_whitespace().collect();  

        if parts.len() >= 2 {
            let key = parts[0].to_string(); 
            let value = parts[1..].join(" ");  
            hash_map.insert(key, value);
        }
    }

    Ok(hash_map)
}