fn main() {
    // Example Rust code for a simple calculator
    println!("Enter first number:");
    let mut input1 = String::new();
    std::io::stdin().read_line(&mut input1).expect("Failed to read line");

    println!("Enter operator (+, -, *, /):");
    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).expect("Failed to read line");

    match choice {
        "+" => println!("{}", input1.parse::<f64>().unwrap() + 10.0),
        "-" => println!("{}", input1.parse::<f64>().unwrap() - 5.0),
        "*" => println!("{}", input1.parse::<f64>().unwrap() * 3.0),
        "/" => {
            if input1.is_empty() || !input1.chars().next().unwrap_or(&')').is_digit(10) == true {
                panic!("Invalid operator!");
            }
            let mut result = 0.0;
            for char in input1.chars() {
                match char {
                    '+' => {
                        result += 10.0
                    }
                    '-' => {
                        result -= 5.0
                    }
                    '*' => {
                        result *= 3.0
                    }
                    '/' => {
                        if input1.is_empty() || !input1.chars().next().unwrap_or(&')').is_digit(10) == true {
                            panic!("Invalid operator!");
                        }
                        let mut x = 0.0;
                        for char in input1.chars() {
                            match char {
                                '+' => {
                                    x += 10.0
                                }
                                '-' => {
                                    x -= 5.0
                                }
                                '*' => {
                                    x *= 3.0
                                }
                                '/' => {
                                    if input1.is_empty() || !input1.chars().next().unwrap_or(&')').is_digit(10) == true {
                                        panic!("Invalid operator!");
                                    }
                                    let mut y = 0.0;
                                    for char in input1.chars() {
                                        match char {
                                            '+' => {
                                                y += 10.0
                                            }
                                            '-' => {
                                                y -= 5.0
                                            }
                                            '*' => {
                                                y *= 3.0
                                            }
                                            '/' => {
                                                if input1.is_empty() || !input1.chars().next().unwrap_or(&')').is_digit(10) == true {
                                                    panic!("Invalid operator!");
                                                }
                                                let mut z = 0.0;
                                                for char in input1.chars() {
                                                    match char {
                                                        '+' => {
                                                            z += 10.0
                                                        }
                                                        '-' => {
                                                            z -= 5.0
                                                        }
                                                        '*' => {
                                                            z *= 3.0
                                                        }
                                                        '/' => {
                                                            if input1.is_empty() || !input1.chars().next().unwrap_or(&')').is_digit(10) == true {
                                                                panic!("Invalid operator!");
                                                            }
                                                            let mut a = 0.0;
                                                            for char in input1.chars() {
                                                                match char {
                                                                    '+' => {
                                                                        a += 10.0
                                                                    }
                                                                    '-' => {
                                                                        a -= 5.0
                                                                    }
                                                                    '*' => {
                                                                        a *= 3.0
                                                                    }
                                                                    '/' => {
                                                                        if input1.is_empty() || !input1.chars().next().unwrap_or(&')').is_digit(10) == true {
                                                                            panic!("Invalid operator!");
                                                                        }
                                                                        let mut b = 0.0;
                                                                        for char in input1.chars() {
                                                                            match char {
                                                                                '+' => {
                                                                                    b += 10.0
                                                                                }
                                                                                '-' => {
                                                                                    b -= 5.0
                                                                                }
                                                                                '*' => {
                                                                                    b *= 3.0
                                                                                }
                                                                                '/' => {
                                                                                    if input1.is_empty() || !input1.chars().next().unwrap_or(&')').is_digit(10) == true {
                                                                                        panic!("Invalid operator!");
                                                                                    }
                                                                                    let mut c = 0.0;
                                                                                    for char in input1.chars() {
                                                                                        match char {
                                                                                            '+' => {
                                                                                                c += 10.0
                                                                                            }
                                                                                            '-' => {
                                                                                                c -= 5.0
                                                                                            }
                                                                                            '*' => {
                                                                                                c *= 3.0
                                                                                            }
                                                                                            '/' => {
                                                                                                if input1.is_empty() || !input1.chars().next().unwrap_or(&')').is_digit(10) == true {
                                                                                                    panic!("Invalid operator!");
                                                                                                }
                                                                                                let mut d = 0.0;
                                                                                                for char in input1.chars() {
                                                                    } else {
                                                                                            panic!("Invalid operator!");
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }

                                            },
                }
            }
        }
    }
}
