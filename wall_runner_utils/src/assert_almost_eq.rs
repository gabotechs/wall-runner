#[macro_export]
macro_rules! assert_almost_eq {
    ($n1: expr, $n2: expr) => {
        match (&$n1, &$n2) {
            (n1, n2) => {
                if (n1 - n2).abs() > 0.0000001 {
                    assert_eq!(n1, n2)
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::assert_almost_eq;

    #[test]
    fn assert_almost_eq() {
        assert_almost_eq!(0.00000001f32, 0.0);
    }
}
