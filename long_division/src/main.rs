fn main() {
    //implement long division
    println!("{:?}", long_division(27, 5));
    println!("{:?}", int_to_binary(10));
}

//Excercise 6
fn long_division(numerator: i32, denominator: i32) -> (i32, i32) {
    //implement long division
    //a = m * b + r
    //a = numerator
    //b = denominator

    // 0 <= r < b

    //edge cases if numerator is 0
    if numerator == 0 {
        return (0, 0);
    }

    //edge cases if denominator is 0
    if denominator == 0 {
        return (0, 0);
    }

    //Example = 27 = m * 5 + r
    //first step divide numerator by denominator
    //here it would be : m = 27 / 5 = 5
    let mut quotient = numerator / denominator;
    //here it would be : r = 27 % 5 = 2
    let mut remainder = numerator % denominator;

    return (quotient, remainder);
}

//Excercise 7
fn int_to_binary(mut n: u32) -> String {
    if n == 0 {
        return "0".to_string();
    }

    let mut binary_representation = String::new();

    while n > 0 {
        let remainder = n % 2;
        binary_representation.insert(0, char::from_digit(remainder, 10).unwrap());
        n /= 2;
    }

    binary_representation
}
