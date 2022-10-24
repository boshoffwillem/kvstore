use std::collections::HashMap;

fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().expect("The key was not there");
    let value = arguments.next().unwrap();
    println!("The key is '{}', and the value is '{}'", key, value);
    let mut database = Database::new().expect("Database::new() crashed");
    database.insert(key.to_uppercase(), value);
}

struct Database {
    map: HashMap<String, String>,
    flush: bool
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let mut map = HashMap::new();
        let contents = match std::fs::read_to_string("kv.db") {
            Ok(c) => c,
            Err(error) => {
                return Result::Err(error);
            }
        };
        for line in contents.lines() {
            let (key, value) = line.split_once('\t').expect("Corrupt database");
            map.insert(key.to_owned(), value.to_owned());
        }
        Ok(Database { map, flush: false })
    }

    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    fn flush(mut self) -> std::io::Result<()> {
        self.flush = true;
        do_flush(&self)
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        if !self.flush {
            let _ = do_flush(self);
        }
    }
}

fn do_flush(database: &Database) -> std::io::Result<()> {
    let mut contents = String::new();
    for (key, value) in &database.map {
        contents.push_str(&key);
        contents.push('\t');
        contents.push_str(&value);
        contents.push('\n');
    }
    std::fs::write("kv.db", contents)
}
