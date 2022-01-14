use serde_json::Value;

/* ---------- */

fn add_numbers(value: &Value) -> i64 {
    match value {
        Value::Array(arr) => arr.iter().map(add_numbers).sum(),
        Value::Object(obj) => obj.iter().map(|(_, val)| add_numbers(val)).sum(),
        Value::Number(val) => val.as_i64().expect("failed to retrieve json value"),
        _ => 0
    }
}

/* ---------- */

fn main() {
    let value: Value = serde_json::from_str(utils::input_str!("part1.json")).expect("failed to parse");
    println!("{}", add_numbers(&value));
}
