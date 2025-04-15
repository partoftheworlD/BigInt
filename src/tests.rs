#[cfg(test)]
use crate::BigInt;

mod tests {
    use super::*;

    #[test]
    fn biguint_test() {
        let bigint = BigInt::bigint(453283475834787583).digits.iter().collect::<String>();
        assert_eq!(bigint, "453283475834787583")
    }

    #[test]
    fn bigint_test() {
        let bigint = BigInt::bigint(-453283475834787583).digits.iter().collect::<String>();
        assert_eq!(bigint, "-453283475834787583")
    }

    #[test]
    fn str_to_big() {
        let bigint = BigInt::str_to_big(
            "4238345356347583658638756345738576347856346538656384756384563486573454747756564675746565745771",
        );

        assert_eq!(
            bigint.digits,
            [
                '4', '2', '3', '8', '3', '4', '5', '3', '5', '6', '3', '4', '7', '5', '8', '3',
                '6', '5', '8', '6', '3', '8', '7', '5', '6', '3', '4', '5', '7', '3', '8', '5',
                '7', '6', '3', '4', '7', '8', '5', '6', '3', '4', '6', '5', '3', '8', '6', '5',
                '6', '3', '8', '4', '7', '5', '6', '3', '8', '4', '5', '6', '3', '4', '8', '6',
                '5', '7', '3', '4', '5', '4', '7', '4', '7', '7', '5', '6', '5', '6', '4', '6',
                '7', '5', '7', '4', '6', '5', '6', '5', '7', '4', '5', '7', '7', '1'
            ]
        )
    }

    #[test]
    fn none_str_to_big() {
        let bigint = BigInt::str_to_big("");
        assert_eq!(bigint.digits, [])
    }

    #[test]
    fn big_mult() {
        let bigint1 = BigInt::str_to_big("1337");
        let bigint2 = BigInt::str_to_big("1321");

        assert_eq!(bigint1 * bigint2, BigInt::str_to_big("1766177"))
    }

    #[test]
    fn really_big_mult() {
        let bigint1 = BigInt::str_to_big("1337534536546454564564");
        let bigint2 = BigInt::str_to_big("4335436455675675656");

        assert_eq!(
            bigint1 * bigint2,
            BigInt::str_to_big("5798795990468768445615598443005975053984")
        )
    }
}
