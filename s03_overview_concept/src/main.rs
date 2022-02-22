fn main() {
    // variable_sample()
    // function_sample()
    control_flow_sample()
}
/*
 * Variable
 * Data type
 */
fn variable_sample() {
    // let x = 20;
    let mut x = 5;
    println!("giá trị x là '{}'", x);
    x = 10;
    println!("giá trị x là '{}'", x);
    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);

    // const y: u32 = 10;
    // println!("giá trị y là '{}'", y);

    let mut a = 0;
    let mut b = 1;
    let c = b;
    b = a;
    a = c;
    println!("giá trị a - '{}', b - '{}', c - '{}'", a, b, c);

    // Integer
    println!("######_Integer_###############");
    let o: i32 = 123_123; // Decimal
    let p: i32 = 0b1111_0000; // Binary
    let q: i32 = 0xff; // hex
    let r: u8 = b'A';
    println!("giá trị o - '{}', p - '{}', q - '{}', r - '{}'", o, p, q, r);
    // Numeric Operations
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 38;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;
    // Float
    println!("######_Float_###############");
    let division: f64 = 4.0 / 3.0;
    let remainder: i32 = 10 / 3;
    println!(
        "giá trị division - '{}', remainder - '{}'",
        division, remainder
    );

    // Boolean
    println!("######_Boolean_###############");
    let b1 = true;
    let b2 = false;
    println!("giá trị b1 - '{}', b2 - '{}'", b1, b2);

    // Character
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // Array
    let a = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    // Specify type like the following: [data type; length]
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // Following will create an array length of 5 with value of 3
    // Same as writing: let a = [3, 3, 3, 3, 3];
    let a = [3; 5];

    let first = a[0];
    let second = a[1];

    let index = 10;
    // Following would throw index out of bound exception
    // let element = a[index];
}
/*
 * Data type
 */
pub fn function_sample() {
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(5);
    println!("The value of x is: {}", x);
}
fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

/*
 * control flow
 */
pub fn control_flow_sample() {
    // If/Else If/Else
    let number = 3;

    if number < 5 {
        println!("Condition was true.");
    } else {
        println!("Condition was false");
    }

    if number != 0 {
        println!("Number was something other than zero.");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("Number is divisible by 4.");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3.");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2.");
    } else {
        println!("Number is not divisible by 4, 3, or 2.");
    }

    let condition = true;
    let number = if condition {
        5
    } else {
        // "six" here would throw an exception, each block must return the same type
        6
    };
    println!("The value of number is: {}", number);

    // Loop (infinite)
    #[allow(clippy::never_loop)]
    loop {
        println!("Again!");

        // We will exit the loop after the first iteration
        break;
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The value of result is: {}", result);

    // While loop
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("Liftoff!!!");

    // For loop
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("The value is: {}", element);
    }

    // With a range
    // Will create an array like of [1, 2, 3], then reverse it and iterate over it
    for number in (1..4).rev() {
        println!("{}!", number);
    }

    println!("Liftoff!!!");
}
