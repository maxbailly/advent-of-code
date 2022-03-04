use serde_json::Value;

/* ---------- */

fn part1(value: &Value) -> i64 {
    match value {
        Value::Array(arr) => arr.iter().map(part1).sum(),
        Value::Object(obj) => obj.iter().map(|(_, val)| part1(val)).sum(),
        Value::Number(val) => val.as_i64().expect("failed to retrieve json value"),
        _ => 0,
    }
}

/* ---------- */

fn part2(value: &Value) -> i64 {
    match value {
        Value::Number(val) => val.as_i64().expect("failed to retrieve json value"),
        Value::Array(arr) => arr.iter().map(part2).sum(),
        Value::Object(obj) => {
            let mut add = 0;

            let ret = obj.iter().try_for_each(|(_, val)| {
                if matches!(val, Value::String(val) if val == "red") {
                    return Err(());
                }

                add += part2(val);

                Ok(())
            });

            if ret.is_err() {
                0
            } else {
                add
            }
        }
        _ => 0,
    }
}

/* ---------- */

fn main() {
    let value: Value =
        serde_json::from_str(utils::input_str!("input.json")).expect("failed to parse");

    utils::answer!(&value);
}

/* ---------- */

#[cfg(test)]
mod tests {
    use serde_json::Value;

    #[test]
    fn part2() {
        // [1,2,3] => 6
        // [1,{"c":"red","b":2},3] => 4
        // {"d":"red","e":[1,2,3,4],"f":5} => 0
        // [1,"red",5] => 6

        let value: Value = serde_json::from_str(r#"[1,2,3]"#).expect("failed to parse");
        assert_eq!(crate::part2(&value), 6);

        let value: Value =
            serde_json::from_str(r#"[1,{"c":"red","b":2},3]"#).expect("failed to parse");
        assert_eq!(crate::part2(&value), 4);

        let value: Value =
            serde_json::from_str(r#"{"d":"red","e":[1,2,3,4],"f":5}"#).expect("failed to parse");
        assert_eq!(crate::part2(&value), 0);

        let value: Value = serde_json::from_str(r#"[1,"red",5]"#).expect("failed to parse");
        assert_eq!(crate::part2(&value), 6);
    }
}
