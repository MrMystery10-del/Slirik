pub fn calculate(operation: &String, x: f64, y: f64) -> f64 {
    match operation.as_str() {
        "+" => add(x, y),
        "-" => sub(x, y),
        "*" => mul(x, y),
        "/" => div(x, y),
        "^" => pow(x, y),
        "#" => x,
        _ => panic!("ERROR | Unexpected operator")
    }
}

fn add(x: f64, y: f64) -> f64 {
    return x + y;
}

fn sub(x: f64, y: f64) -> f64 {
    return x - y;
}

fn mul(x: f64, y: f64) -> f64 {
    return x * y;
}

fn div(x: f64, y: f64) -> f64 {
    return x / y;
}

fn pow(x: f64, y: f64) -> f64 {
    let mut result = x;
    let mut index = 1;

    while index < y as i64 {
        result *= x;
        index += 1;
    }
    return result;
}