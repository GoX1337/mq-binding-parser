use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use std::collections::HashMap;

#[derive(Debug)]
struct Object {
    name: String,
    class_name: String,
    factory_name: String,
    params: HashMap<String, String>
}
impl Object {
    fn new() -> Object {
        Object{
            name:"".to_string(),
            class_name:"".to_string(),
            factory_name:"".to_string(),
            params: HashMap::new()
        }
    }
}

fn main() -> Result<(), Error> {
    let path = ".bindings";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    let objects = build_mq_map(buffered);
    print_mq_map(&objects);
    Ok(())
}

fn build_mq_map(buffered: BufReader<File>) -> HashMap<String, Object> {
    let mut objects = HashMap::new();
    let mut last_param_name = "".to_string();

    for line in buffered.lines() {
        if let Ok(obj) = line {
            let tokens: Vec<&str> = obj.split('/').collect();
            let name = tokens[0].to_string();

            if obj.contains("ClassName") {
                let classname: Vec<&str> = tokens[1].split('=').collect();
                let object = Object{name: name.clone(), class_name: classname[1].to_string(), factory_name: "".to_string(), params: HashMap::new()};
                objects.insert(name, object);
            } else if obj.contains("FactoryName") {
                let factory: Vec<&str> = tokens[1].split('=').collect();
                let factory_name = factory[1].to_string();
                objects.entry(name).or_insert(Object::new()).factory_name = factory_name;
            } else if tokens.len() > 3 && tokens[3].starts_with("Type") {
                let val: Vec<&str> = tokens[3].split('=').collect();
                last_param_name = val[1].to_string();
            } else if tokens.len() > 3 && tokens[3].starts_with("Content") {
                let val: Vec<&str> = tokens[3].split('=').collect();
                if !val[1].to_string().is_empty() {
                    objects.entry(name).or_insert(Object::new()).params.insert(last_param_name.to_string(), val[1].to_string());
                }
            }
        }
    }
    objects
}

fn print_mq_map(objects: &HashMap<String, Object>) {
    for (name, object) in objects.iter() {
        println!("{}", name);
        for (param, value) in object.params.iter() {
            println!("  {}: {}", param, value);
        }
        println!("");
    }
}