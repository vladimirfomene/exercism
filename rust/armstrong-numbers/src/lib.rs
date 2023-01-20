pub fn is_armstrong_number(num: u32) -> bool {
    let number_of_digits = number_of_digits(num);
    check_for_armstrong_number(number_of_digits, num)
}

fn number_of_digits(mut num: u32) -> u32 {
    let mut digits = 0u32;
    if num == 0 {
        return 1;
    }

    while num != 0  {
        num = num / 10;
        digits += 1;
    }

    digits
}

fn check_for_armstrong_number(number_of_digits: u32, to_test: u32) -> bool {
    let mut sum = 0u64;
    let mut num = to_test;
    while num != 0 {
        let digit = (num % 10) as u64;
        num = num / 10;
        sum += digit.pow(number_of_digits);
    }

    sum == to_test as u64
}
