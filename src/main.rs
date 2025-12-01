fn main() {
    println!("Hello, world!");
}

pub fn fizz_buzz(num: i32) -> String {
    match (num % 3 == 0, num % 5 == 0) {
        (true, true) => "FizzBuzz".to_string(),
        (true, false) => "Fizz".to_string(),
        (false, true) => "Buzz".to_string(),
        _ => num.to_string(),
    }
}

// Fizzbuzz問題
// 1. 数が3で割り切れる時、「Fizz」を出力する。
// 2. 数が5で割り切れる時、「Buzz」を出力する。
// 3. 数が3でも5でも割り切れる時、「FizzBuzz」を出力する。
// 4. 上記以外の数値の場合、渡された数値をそのまま出力する。
//
#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn 数が3で割り切れる時_Fizzを出力する() {
        assert_eq!(fizz_buzz(3), "Fizz");
    }

    #[test]
    fn 数が5で割り切れる時_Buzzを出力する() {
        assert_eq!(fizz_buzz(5), "Buzz");
    }

    #[test]
    fn 数が3でも5でも割り切れる時_FizzBuzzを出力する() {
        assert_eq!(fizz_buzz(15), "FizzBuzz");
    }

    #[test]
    fn 数が3でも5でも割り切れない場合_そのまま数値を出力する() {
        assert_eq!(fizz_buzz(7), "7")
    }
}
