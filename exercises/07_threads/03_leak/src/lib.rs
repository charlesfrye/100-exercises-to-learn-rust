use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    let v = v.leak();
    let (left, right) = v.split_at(v.len() / 2);

    let h_left = thread::spawn(move || -> i32 { left.iter().sum() });
    let h_right = thread::spawn(move || -> i32 { right.iter().sum() });

    h_left.join().unwrap() + h_right.join().unwrap()
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
