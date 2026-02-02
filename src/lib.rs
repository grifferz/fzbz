// Calls the implementation of fizzbuzz (fzbz_fn) on each integer from 1 up to the desired maximum
// and return the output in a Vec.
pub fn fizzbuzz_all(fzbz_fn: fn(i32) -> Answer, max: i32) -> Vec<Answer> {
    // TODO: Check if this is smart enough to allocate the Vec only once, not keep growing it as it
    // collects.
    (1..=max).map(fzbz_fn).collect()
}

#[derive(Debug, PartialEq)]
pub enum Answer {
    Buzz,
    Fizz,
    FizzBuzz,
    Number(i32),
}

// Just a straightforward test of every scenario.
pub fn naive(n: i32) -> Answer {
    // Maybe it would be more naive to convert to string at every test but it seems fairly obvious
    // that it's always going to be the same string.
    let n_str = n.to_string();

    if (n % 5 == 0 || n_str.contains('5')) && (n % 7 == 0 || n_str.contains('7')) {
        Answer::FizzBuzz
    } else if n % 5 == 0 || n_str.contains('5') {
        Answer::Buzz
    } else if n % 7 == 0 || n_str.contains('7') {
        Answer::Fizz
    } else {
        Answer::Number(n)
    }
}

// Make it look cleaner by doing the modulus and string match first and then using a match to check
// all possible states.
pub fn mod_then_match(n: i32) -> Answer {
    let n_str = n.to_string();

    let buzzy = n % 5 == 0 || n_str.contains('5');
    let fizzy = n % 7 == 0 || n_str.contains('7');

    match (buzzy, fizzy) {
        (true, true) => Answer::FizzBuzz,
        (true, _) => Answer::Buzz,
        (_, true) => Answer::Fizz,
        _ => Answer::Number(n),
    }
}

// Same as `mod_then_match` but check for the case where both '5' and '7' appear in the number
// string and short circuit that before doing any modulus tests.
pub fn early_return_before_mod(n: i32) -> Answer {
    let n_str = n.to_string();

    let buzzy = n_str.contains('5');
    let fizzy = n_str.contains('7');

    if buzzy && fizzy {
        return Answer::FizzBuzz;
    }

    let buzzy = buzzy || n % 5 == 0;
    let fizzy = fizzy || n % 7 == 0;

    match (buzzy, fizzy) {
        (true, true) => Answer::FizzBuzz,
        (true, _) => Answer::Buzz,
        (_, true) => Answer::Fizz,
        _ => Answer::Number(n),
    }
}

// Like `early_return_before_mod` but check string contents by doing one loop through it, inctead
// of multiple `contains()` checks.
pub fn single_string_scan(n: i32) -> Answer {
    let n_str = n.to_string();

    let mut buzzy = false;
    let mut fizzy = false;

    for c in n_str.chars() {
        if c == '5' {
            buzzy = true;
        } else if c == '7' {
            fizzy = true;
        }
    }

    let buzzy = buzzy || n % 5 == 0;
    let fizzy = fizzy || n % 7 == 0;

    match (buzzy, fizzy) {
        (true, true) => Answer::FizzBuzz,
        (true, _) => Answer::Buzz,
        (_, true) => Answer::Fizz,
        _ => Answer::Number(n),
    }
}

// Like `single_string_scan` but do a check for the "FizzBuzz" case due to character matches, to
// sometimes avoid having to do modulus checks.
pub fn single_string_scan_early_fizzbuzz(n: i32) -> Answer {
    let n_str = n.to_string();

    let mut buzzy = false;
    let mut fizzy = false;

    for c in n_str.chars() {
        if c == '5' {
            buzzy = true;
            if fizzy {
                return Answer::FizzBuzz;
            }
        } else if c == '7' {
            fizzy = true;
            if buzzy {
                return Answer::FizzBuzz;
            }
        }
    }

    if buzzy && fizzy {
        return Answer::FizzBuzz;
    }

    let buzzy = buzzy || n % 5 == 0;
    let fizzy = fizzy || n % 7 == 0;

    match (buzzy, fizzy) {
        (true, true) => Answer::FizzBuzz,
        (true, _) => Answer::Buzz,
        (_, true) => Answer::Fizz,
        _ => Answer::Number(n),
    }
}

// Don't do any conversion to a string; instead check each digit of `n` with modulus operator.
pub fn only_using_mod(n: i32) -> Answer {
    let (buzzy, fizzy) = test_for_fives_and_sevens(n);

    let buzzy = buzzy || n % 5 == 0;
    let fizzy = fizzy || n % 7 == 0;

    match (buzzy, fizzy) {
        (true, true) => Answer::FizzBuzz,
        (true, _) => Answer::Buzz,
        (_, true) => Answer::Fizz,
        _ => Answer::Number(n),
    }
}

fn test_for_fives_and_sevens(mut n: i32) -> (bool, bool) {
    let mut five = false;
    let mut seven = false;

    // Doing `n % 10` separates off the last (right-most, least significant) digit, then dividing
    // by 10 lops off that digit and lets us consider the next one. e.g. given `n = 4567`:
    // n % 10 = 7
    // seven = true
    // n / 10 = 456
    // n % 10 = 6
    // n / 10 = 45
    // n % 10 = 5
    // five = true
    // n / 10 = 4
    // n % 10 = 4
    // n / 10 = 0, stop there returning (true, true).
    while n > 0 {
        let digit = n % 10;

        match digit {
            5 => five = true,
            7 => seven = true,
            _ => {}
        };

        n /= 10;
    }

    (five, seven)
}

pub fn only_using_mod_with_early_return(n: i32) -> Answer {
    let (buzzy, fizzy) = test_for_fives_and_sevens_with_early_return(n);

    let buzzy = buzzy || n % 5 == 0;
    let fizzy = fizzy || n % 7 == 0;

    match (buzzy, fizzy) {
        (true, true) => Answer::FizzBuzz,
        (true, _) => Answer::Buzz,
        (_, true) => Answer::Fizz,
        _ => Answer::Number(n),
    }
}

// Like `test_for_fives_and_sevens` but return as soon as both 5 and 7 are found.
fn test_for_fives_and_sevens_with_early_return(mut n: i32) -> (bool, bool) {
    let mut five = false;
    let mut seven = false;

    // Doing `n % 10` separates off the last (right-most, least significant) digit, then dividing
    // by 10 lops off that digit and lets us consider the next one. e.g. given `n = 4567`:
    // n % 10 = 7
    // seven = true
    // n / 10 = 456
    // n % 10 = 6
    // n / 10 = 45
    // n % 10 = 5
    // five = true
    // stop here as both five and seven are true, returning (true, true)
    while n > 0 {
        let digit = n % 10;

        match digit {
            5 => {
                five = true;
                if seven {
                    return (true, true);
                }
            }
            7 => {
                seven = true;
                if five {
                    return (true, true);
                }
            }
            _ => {}
        };

        n /= 10;
    }

    (five, seven)
}
