use fzbz::*;

// An exploration of a few different implementations of a specific brand of FizzBuzz game as
// described by Andy Balaam in his video at:
//         https://video.infosec.exchange/w/2dEHo81R7ozrfohX2PARWt
//
// This particular brand of FizzBuzz game works like this:
// - If the number is divisible by 5 then that's a Buzz.
// - If the number has a 5 anywhere in it then that's a Buzz.
// - If the number is divisible by 7 then that's a Buzz.
// - If the number has a 7 anywhere in it then that's a Fizz.
// - If the number is both a Buzz and a Fizz then it's actually a FizzBuzz.
// - Otherwise it's just a number.
fn main() {
    // Do fizzbuzz up to this number.
    let max: i32 = 2_000_000;

    // First argument is the function that implements fizzbuzz.
    let answers = fizzbuzz_all(only_using_mod_with_early_return, max);

    // Count up the different kinds of answers just for something to do.
    // TODO: Might be interesting to find a way to add some tracing so as to know by what means
    // each kind of answer was obtained, e.g. count of "FizzBuzz" that happened because it was
    // divisible by 5 and contained a '7' as distinct from the other ways to get "FizzBuzz".
    let (buzz_count, fizz_count, fizzbuzz_count, number_count) =
        answers
            .iter()
            .fold((0, 0, 0, 0), |(b, f, fb, n), a| match a {
                Answer::Buzz => (b + 1, f, fb, n),
                Answer::Fizz => (b, f + 1, fb, n),
                Answer::FizzBuzz => (b, f, fb + 1, n),
                Answer::Number(_) => (b, f, fb, n + 1),
            });

    println!("Fizzbuzz({max}) resulted in:");
    println!("\tBuzz:     {buzz_count} ({:.1}%)", pct(buzz_count, max));
    println!("\tFizz:     {fizz_count} ({:.1}%)", pct(fizz_count, max));
    println!(
        "\tFizzBuzz: {fizzbuzz_count} ({:.1}%)",
        pct(fizzbuzz_count, max)
    );
    println!(
        "\tNumber:   {number_count} ({:.1}%)",
        pct(number_count, max)
    );
}

fn pct(n: i32, max: i32) -> f32 {
    if n == 0 || max == 0 {
        0.0
    } else {
        n as f32 / max as f32 * 100.0
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::Answer::*;
    use super::*;

    // Run all the tests for each implementation that we have of fizzbuzz.

    #[test_case(naive ; "using naive implementation")]
    #[test_case(mod_then_match ; "using mod first then match cases implementation")]
    #[test_case(early_return_before_mod ; "using early return before mod")]
    #[test_case(single_string_scan ; "using single string scan")]
    #[test_case(single_string_scan_early_fizzbuzz ; "using single string scan with early fizzbuzz shortcircuit")]
    #[test_case(only_using_mod ; "using mod instead of any string conversion")]
    #[test_case(only_using_mod_with_early_return ; "using mod instead of any string conversion, early return")]
    fn low_numbers_are_numbers(fzbz_fn: fn(i32) -> Answer) {
        assert_eq!(fzbz_fn(1), Number(1));
        assert_eq!(fzbz_fn(2), Number(2));
        assert_eq!(fzbz_fn(3), Number(3));
        assert_eq!(fzbz_fn(4), Number(4));
    }

    #[test_case(naive ; "using naive implementation")]
    #[test_case(mod_then_match ; "using mod first then match cases implementation")]
    #[test_case(early_return_before_mod ; "using early return before mod")]
    #[test_case(single_string_scan ; "using single string scan")]
    #[test_case(single_string_scan_early_fizzbuzz ; "using single string scan with early fizzbuzz shortcircuit")]
    #[test_case(only_using_mod ; "using mod instead of any string conversion")]
    #[test_case(only_using_mod_with_early_return ; "using mod instead of any string conversion, early return")]
    fn multiples_of_five_are_buzz(fzbz_fn: fn(i32) -> Answer) {
        assert_eq!(fzbz_fn(5), Buzz);
        assert_eq!(fzbz_fn(10), Buzz);
        assert_eq!(fzbz_fn(15), Buzz);
        assert_eq!(fzbz_fn(100), Buzz);
    }

    #[test_case(naive ; "using naive implementation")]
    #[test_case(mod_then_match ; "using mod first then match cases implementation")]
    #[test_case(early_return_before_mod ; "using early return before mod")]
    #[test_case(single_string_scan ; "using single string scan")]
    #[test_case(single_string_scan_early_fizzbuzz ; "using single string scan with early fizzbuzz shortcircuit")]
    #[test_case(only_using_mod ; "using mod instead of any string conversion")]
    #[test_case(only_using_mod_with_early_return ; "using mod instead of any string conversion, early return")]
    fn multiples_of_seven_are_fizz(fzbz_fn: fn(i32) -> Answer) {
        assert_eq!(fzbz_fn(7), Fizz);
        assert_eq!(fzbz_fn(14), Fizz);
        assert_eq!(fzbz_fn(21), Fizz);
        assert_eq!(fzbz_fn(84), Fizz);
    }

    #[test_case(naive ; "using naive implementation")]
    #[test_case(mod_then_match ; "using mod first then match cases implementation")]
    #[test_case(early_return_before_mod ; "using early return before mod")]
    #[test_case(single_string_scan ; "using single string scan")]
    #[test_case(single_string_scan_early_fizzbuzz ; "using single string scan with early fizzbuzz shortcircuit")]
    #[test_case(only_using_mod ; "using mod instead of any string conversion")]
    #[test_case(only_using_mod_with_early_return ; "using mod instead of any string conversion, early return")]
    fn higher_numbers_between_fizzes_and_buzzes_are_numbers(fzbz_fn: fn(i32) -> Answer) {
        assert_eq!(fzbz_fn(6), Number(6));
        assert_eq!(fzbz_fn(8), Number(8));
        assert_eq!(fzbz_fn(9), Number(9));
        assert_eq!(fzbz_fn(11), Number(11));
        assert_eq!(fzbz_fn(12), Number(12));
        assert_eq!(fzbz_fn(13), Number(13));
        assert_eq!(fzbz_fn(16), Number(16));
    }

    #[test_case(naive ; "using naive implementation")]
    #[test_case(mod_then_match ; "using mod first then match cases implementation")]
    #[test_case(early_return_before_mod ; "using early return before mod")]
    #[test_case(single_string_scan ; "using single string scan")]
    #[test_case(single_string_scan_early_fizzbuzz ; "using single string scan with early fizzbuzz shortcircuit")]
    #[test_case(only_using_mod ; "using mod instead of any string conversion")]
    #[test_case(only_using_mod_with_early_return ; "using mod instead of any string conversion, early return")]
    fn multiples_of_both_are_fizzbuzz(fzbz_fn: fn(i32) -> Answer) {
        assert_eq!(fzbz_fn(35), FizzBuzz);
        assert_eq!(fzbz_fn(70), FizzBuzz);
        assert_eq!(fzbz_fn(105), FizzBuzz);
    }

    #[test_case(naive ; "using naive implementation")]
    #[test_case(mod_then_match ; "using mod first then match cases implementation")]
    #[test_case(early_return_before_mod ; "using early return before mod")]
    #[test_case(single_string_scan ; "using single string scan")]
    #[test_case(single_string_scan_early_fizzbuzz ; "using single string scan with early fizzbuzz shortcircuit")]
    #[test_case(only_using_mod ; "using mod instead of any string conversion")]
    #[test_case(only_using_mod_with_early_return ; "using mod instead of any string conversion, early return")]
    fn mentioning_five_makes_you_a_buzz(fzbz_fn: fn(i32) -> Answer) {
        assert_eq!(fzbz_fn(51), Buzz);
        assert_eq!(fzbz_fn(52), Buzz);
        assert_eq!(fzbz_fn(53), Buzz);
        assert_eq!(fzbz_fn(54), Buzz);
    }

    #[test_case(naive ; "using naive implementation")]
    #[test_case(mod_then_match ; "using mod first then match cases implementation")]
    #[test_case(early_return_before_mod ; "using early return before mod")]
    #[test_case(single_string_scan ; "using single string scan")]
    #[test_case(single_string_scan_early_fizzbuzz ; "using single string scan with early fizzbuzz shortcircuit")]
    #[test_case(only_using_mod ; "using mod instead of any string conversion")]
    #[test_case(only_using_mod_with_early_return ; "using mod instead of any string conversion, early return")]
    fn mentioning_seven_makes_you_a_fizz(fzbz_fn: fn(i32) -> Answer) {
        assert_eq!(fzbz_fn(17), Fizz);
        assert_eq!(fzbz_fn(27), Fizz);
        assert_eq!(fzbz_fn(71), Fizz);
    }

    #[test_case(naive ; "using naive implementation")]
    #[test_case(mod_then_match ; "using mod first then match cases implementation")]
    #[test_case(early_return_before_mod ; "using early return before mod")]
    #[test_case(single_string_scan ; "using single string scan")]
    #[test_case(single_string_scan_early_fizzbuzz ; "using single string scan with early fizzbuzz shortcircuit")]
    #[test_case(only_using_mod ; "using mod instead of any string conversion")]
    #[test_case(only_using_mod_with_early_return ; "using mod instead of any string conversion, early return")]
    fn all_combinations_of_fizz_and_buzz_make_fizzbuzz(fzbz_fn: fn(i32) -> Answer) {
        assert_eq!(fzbz_fn(70), FizzBuzz);
        assert_eq!(fzbz_fn(35), FizzBuzz);
        assert_eq!(fzbz_fn(56), FizzBuzz);
        assert_eq!(fzbz_fn(57), FizzBuzz);
        assert_eq!(fzbz_fn(570), FizzBuzz);
    }

    #[test_case(naive ; "using naive implementation")]
    #[test_case(mod_then_match ; "using mod first then match cases implementation")]
    #[test_case(early_return_before_mod ; "using early return before mod")]
    #[test_case(single_string_scan ; "using single string scan")]
    #[test_case(single_string_scan_early_fizzbuzz ; "using single string scan with early fizzbuzz shortcircuit")]
    #[test_case(only_using_mod ; "using mod instead of any string conversion")]
    #[test_case(only_using_mod_with_early_return ; "using mod instead of any string conversion, early return")]
    fn fizzbuzz_all_counts_up_to_max(fzbz_fn: fn(i32) -> Answer) {
        let answers = fizzbuzz_all(fzbz_fn, 50);
        assert_eq!(answers[0], Number(1));
        assert_eq!(answers[4], Buzz);
        assert_eq!(answers[6], Fizz);
        assert_eq!(answers[34], FizzBuzz);
        assert_eq!(answers[35], Number(36));
        assert_eq!(answers.len(), 50);
    }
}
