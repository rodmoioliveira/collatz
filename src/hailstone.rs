use num::bigint::BigUint;
use num::Integer;

pub fn sequence(number: u128) -> Vec<BigUint> {
    if number == 0_u128 {
        return vec![];
    }

    let mut steps: Vec<BigUint> = Vec::with_capacity(64);
    let mut next: BigUint = BigUint::from(number);
    let zero = num::zero();
    let one = num::one();
    let two = &one + &one;
    let three = &two + &one;

    while next != one {
        match next.mod_floor(&two) == zero {
            true => {
                next /= &two;
            }
            false => {
                next = (&next * &three) + &one;
            }
        }
        steps.push(next.clone());
    }

    steps
}

#[cfg(test)]
mod tests {
    use super::*;
    use num::bigint::BigUint;

    #[test]
    fn test_sequence() {
        let seq = sequence(10);
        let answer: [u128; 6] = [5, 16, 8, 4, 2, 1];

        assert_eq!(
            seq,
            answer
                .into_iter()
                .map(BigUint::from)
                .collect::<Vec<BigUint>>()
        );
    }
}
