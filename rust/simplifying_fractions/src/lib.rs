pub fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn simplify_fraction(numerator: u32, denominator: u32) -> (u32, u32) {
    let gcd = gcd(numerator, denominator);
    (numerator / gcd, denominator / gcd)
}

#[cfg(test)]
mod tests {
    use gcd;
    use simplify_fraction;

    #[test]
    fn gcd_works() {
        assert_eq!(4, gcd(4, 8));
        assert_eq!(4, gcd(8, 4));
    }

    #[test]
    fn simplify_fraction_works() {
        assert_eq!((1, 2), simplify_fraction(4, 8));
        assert_eq!((64, 3265), simplify_fraction(1536, 78360));
        assert_eq!((25739, 2768), simplify_fraction(51478, 5536));
        assert_eq!((7, 18), simplify_fraction(46410, 119340));
        assert_eq!((7673, 4729), simplify_fraction(7673, 4729));
        assert_eq!((4, 1), simplify_fraction(4096, 1024));
    }
}
