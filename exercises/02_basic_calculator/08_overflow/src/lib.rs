// Customize the `dev` profile to wrap around on overflow.
// Check Cargo's documentation to find out the right syntax:
// https://doc.rust-lang.org/cargo/reference/profiles.html
//
// For reasons that we'll explain later, the customization needs to be done in the `Cargo.toml`
// at the root of the repository, not in the `Cargo.toml` of the exercise.

pub fn factorial(n: u32) -> u32 {
    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn twentieth() {
        // 20! is 2432902008176640000, which is too large to fit in a u32
        let correct: u64 = 2_432_902_008_176_640_000;
        //                           ☝️
        // A large number literal using underscores to improve readability!
        let correct_wrapped: u32 = (correct % (u32::MAX as u64 + 1)).try_into().unwrap();
        assert_eq!(factorial(20), correct_wrapped);
    }

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
