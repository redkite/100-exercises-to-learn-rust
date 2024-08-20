use std::thread;

fn sub_sum(slice: &'static [i32]) -> i32 {
    let mut sum: i32 = 0;
    for val in slice {
        sum += val;
    }

    sum
}

pub fn sum(slice: &'static [i32]) -> i32 {
    let t1 = thread::spawn(|| sub_sum(&slice[(slice.len() / 2)..]));
    let t2 = thread::spawn(|| sub_sum(&slice[..(slice.len() / 2)]));
    t1.join().unwrap() + t2.join().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        static ARRAY: [i32; 0] = [];
        assert_eq!(sum(&ARRAY), 0);
    }

    #[test]
    fn one() {
        static ARRAY: [i32; 1] = [1];
        assert_eq!(sum(&ARRAY), 1);
    }

    #[test]
    fn five() {
        static ARRAY: [i32; 5] = [1, 2, 3, 4, 5];
        assert_eq!(sum(&ARRAY), 15);
    }

    #[test]
    fn nine() {
        static ARRAY: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(sum(&ARRAY), 45);
    }

    #[test]
    fn ten() {
        static ARRAY: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(sum(&ARRAY), 55);
    }
}
