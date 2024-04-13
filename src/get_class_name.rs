use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn get_class_name() -> io::Result<()> {
    let file = File::open("/Users/hamada/workspace/ical-test/src/syllabus.txt")?;
    let reader = BufReader::new(file);

    let mut map = HashMap::new();

    for line in reader.lines() {
        let line = line?; 
        let parts: Vec<&str> = line.split_whitespace().collect();  

        if parts.len() >= 2 {
            let key = parts[0].to_string(); 
            let value = parts[1..].join(" ");  
            map.insert(key, value);
        }
    }

    for (key, value) in &map {
        println!("{} {}", key, value);
    }

    Ok(())
}
