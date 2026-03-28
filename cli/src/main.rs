use core_logic::generate_color;

fn main() {
    let payload = generate_color();

    match serde_json::to_string(&payload) {
        Ok(json_string) => println!("{}", json_string),
        Err(e) => println!("Error formatting JSON: {}", e),
    }
}
