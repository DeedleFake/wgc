use std::cmp::{Ordering};

#[derive(Eq, PartialEq, Debug)]
pub struct Length(pub String);

impl Ord for Length {
    fn cmp(&self, other: &Self) -> Ordering {
        use std::cmp::Ordering::*;

        let len = self.0.len().cmp(&other.0.len());
        match len {
            Equal => self.0.cmp(&other.0).reverse(),
            _ => len,
        }
    }
}

impl PartialOrd for Length {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_length() {
        use super::{Length};
        use std::cmp::{Ordering};

        let short = Length("short".to_string());
        let longer = Length("longer".to_string());
        let equal = Length("equal".to_string());

        assert_eq!(short.cmp(&longer), Ordering::Less);
        assert_eq!(longer.cmp(&short), Ordering::Greater);
        assert_eq!(short.cmp(&equal), Ordering::Equal);
    }
}
