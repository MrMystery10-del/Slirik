pub fn calculate(operation: &String, x: f64, y: f64) -> f64 {
    match operation.as_str() {
        "+" => add(x, y),
        "-" => sub(x, y),
        "*" => mul(x, y),
        "/" => div(x, y),
        "%" => rem(x, y),
        "^" => pow(x, y),
        "#" => root(x, y),
        _ => panic!("ERROR | Unexpected operator")
    }
}

fn add(x: f64, y: f64) -> f64 {
    x + y
}

fn sub(x: f64, y: f64) -> f64 {
    x - y
}

fn mul(x: f64, y: f64) -> f64 {
    x * y
}

fn div(x: f64, y: f64) -> f64 {
    x / y
}

fn rem(x: f64, y: f64) -> f64 {
    x % y
}

fn pow(x: f64, y: f64) -> f64 {
    return x.powf(y);
}

fn root(x: f64, y: f64) -> f64 {
    return y.powf(1.0 / x);
}