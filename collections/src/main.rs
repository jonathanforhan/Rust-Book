use std::fs;

fn json_to_string(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let file = fs::read_to_string(path)?;
    Ok(file)
}

fn main() {
    println!("{}", json_to_string("./extern/User.json").unwrap());
}
