

fn total_len_2(s1: &str, s2: &str) -> usize {
    s1.len() + s2.len()
}

fn total_len_vec(strings: &Vec<&str>) -> usize {
    let mut result = 0;
    for s in strings {
        result += s.len();
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_total_len_2() {
        assert_eq!(2, total_len_2("a", "b"));
        assert_eq!(5, total_len_2("ab", "xyz"));
    }

    #[test]
    fn test_total_len_vec() {
        assert_eq!(2, total_len_vec(&vec!["a", "b"]));
        assert_eq!(6, total_len_vec(&vec!["a", "bc", "def"]));
        assert_eq!(0, total_len_vec(&vec![]));
        assert_eq!(3, total_len_vec(&vec!["xyz"]));
    }
}