// TODO: Define a new trait, `Power`, that has a method `power` that raises `self`
//  to the power of `n`.
//  The trait definition and its implementations should be enough to get
//  the tests to compile and pass.
//
// Recommendation: you may be tempted to write a generic implementation to handle
// all cases at once. However, this is fairly complicated and requires the use of
// additional crates (i.e. `num-traits`).
// Even then, it might be preferable to use a simple macro instead to avoid
// the complexity of a highly generic implementation. Check out the
// "Little book of Rust macros" (https://veykril.github.io/tlborm/) if you're
// interested in learning more about it.
// You don't have to though: it's perfectly okay to write three separate
// implementations manually. Venture further only if you're curious.

macro_rules! impl_power {
    ($type:ty, $exp:ty) => {
        impl Power<$exp> for $type {
            type Output = $type;
            fn power(self, exp: $exp) -> Self::Output {
                self.pow(exp.into())
            }
        }
    };
    ($type:ty, $exp:ty, $exp_ref:ty) => {
        impl Power<$exp_ref> for $type {
            type Output = $type;
            fn power(self, exp: $exp_ref) -> Self::Output {
                self.pow(*exp)
            }
        }
    };
}

trait Power<Rhs = Self> {
    type Output;
    fn power(self, exp: Rhs) -> Self::Output;
}

impl_power!(u32, u16);
impl_power!(u32, u32);
impl_power!(u32, u32, &u32);

#[cfg(test)]
mod tests {
    use super::Power;

    #[test]
    fn test_power_u16() {
        let x: u32 = 2_u32.power(3u16);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_u32() {
        let x: u32 = 2_u32.power(3u32);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_ref_u32() {
        let x: u32 = 2_u32.power(&3u32);
        assert_eq!(x, 8);
    }
}
