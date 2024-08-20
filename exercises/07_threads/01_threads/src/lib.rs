use std::thread;

pub fn sub_sum(v: Vec<i32>) -> i32 {
    let mut sum: i32 = 0;
    for val in v {
        sum += val;
    }
    sum
}

pub fn sum(mut v: Vec<i32>) -> i32 {
    let v2 = v.split_off(v.len() / 2);
    let mut sum: i32 = 0;
    let t1 = thread::spawn(|| sub_sum(v));
    let t2 = thread::spawn(|| sub_sum(v2));

    sum += t1.join().unwrap();
    println!("{sum}");
    sum += t2.join().unwrap();
    println!("{sum}");

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
