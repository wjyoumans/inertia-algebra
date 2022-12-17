include!("../examples/wrapping.rs");

#[macro_use]
extern crate quickcheck;
use quickcheck::{Arbitrary, Gen};

impl<T: Arbitrary> Arbitrary for Wrap<T> {
    fn arbitrary<G: Gen>(g: &mut G) -> Wrap<T> {
        Wrap(<T>::arbitrary(g))
    }
}

mod tests {
    use super::*;

    quickcheck! {
        fn is_ring(arg: (Wrap<i8>,)) -> bool {
            // TODO
            false
        }
    }
}
