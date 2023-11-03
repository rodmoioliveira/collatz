pub fn sequence(number: u32) -> Vec<u128> {
    let mut steps: Vec<u128> = Vec::with_capacity(32);
    let mut next = number as u128;

    while next != 1 {
        match next % 2 == 0 {
            true => {
                next /= 2;
            }
            false => {
                next = (next * 3) + 1;
            }
        }
        steps.push(next);
    }

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence() {
        let seq = sequence(10);
        assert_eq!(seq, vec![5, 16, 8, 4, 2, 1]);
    }
}
