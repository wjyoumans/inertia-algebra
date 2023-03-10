use inertia_algebra::*;
use inertia_algebra::structures::*;
use inertia_algebra::properties::*;

use std::marker::PhantomData;

/// Ring of primitive integers wrapping around at the boundary.
#[derive(Clone, Debug)]
pub struct WrappingRing<T>(PhantomData<T>);

impl<T> WrappingRing<T> {
    pub fn init() -> Self {
        WrappingRing(PhantomData)
    }

    pub fn new(&self, src: T) -> Wrap<T> {
        Wrap(src)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Wrap<T>(T);

macro_rules! impl_wrapping {
    ($($t:ident)*) => ($(

        impl Parent for WrappingRing<$t> {
            type Element = Wrap<$t>;
        }
        
        impl Element for Wrap<$t> {
            type Parent = WrappingRing<$t>;
            fn parent(&self) -> Self::Parent {
                WrappingRing::init()
            }
        }

        // Additive properties

        impl Operation<Additive> for Wrap<$t> {
            fn operate(&self, right: &Self) -> Self {
                Wrap(self.0.wrapping_add(right.0))
            }
        }
        
        impl Identity<Additive> for WrappingRing<$t> {
            fn identity(&self) -> Wrap<$t> {
                Wrap(0)
            }
        }
        
        impl IsIdentity<Additive> for Wrap<$t> {
            fn is_identity(&self) -> bool {
                self.0 == 0
            }
        }

        impl TwoSidedInverse<Additive> for Wrap<$t> {
            fn two_sided_inverse(&self) -> Self {
                Wrap(self.0.wrapping_neg())
            }
        }
        
        impl Divisible<Additive> for WrappingRing<$t> {}
        
        impl Associative<Additive> for WrappingRing<$t> {}
        
        impl Commutative<Additive> for WrappingRing<$t> {}
        
        // Multiplicative properties

        impl Operation<Multiplicative> for Wrap<$t> {
            fn operate(&self, right: &Self) -> Self {
                Wrap(self.0.wrapping_mul(right.0))
            }
        }

        impl Identity<Multiplicative> for WrappingRing<$t> {
            fn identity(&self) -> Wrap<$t> {
                Wrap(1)
            }
        }
        
        impl IsIdentity<Multiplicative> for Wrap<$t> {
            fn is_identity(&self) -> bool {
                self.0 == 1
            }
        }
        
        impl Associative<Multiplicative> for WrappingRing<$t> {}
        
        impl Commutative<Multiplicative> for WrappingRing<$t> {}
       
        // Ring-like properties

        impl Distributive for WrappingRing<$t> {}
    )*);
}

impl_wrapping! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }

fn main() {
    let zn = WrappingRing::<i8>::init();
    
    assert!(zn.is_abstract_magma(Additive));
    assert!(zn.is_abstract_quasigroup(Additive));
    assert!(zn.is_abstract_semigroup(Additive));
    assert!(zn.is_abstract_loop(Additive));
    assert!(zn.is_abstract_group(Additive));
    assert!(zn.is_abstract_group_abelian(Additive));

    assert!(zn.is_abstract_magma(Multiplicative));
    assert!(zn.is_abstract_semigroup(Multiplicative));
    
    assert!(zn.is_abstract_ncring(Additive, Multiplicative));
    assert!(zn.is_abstract_ring(Additive, Multiplicative));
    
    //assert!(zn.is_ncring());
    //assert!(zn.is_ring());

    // not defined since no multiplicative inverses
    //assert!(zn.is_quasigroup(Multiplicative));
    //assert!(zn.is_loop(Multiplicative));
    //assert!(zn.is_group(Multiplicative));
    //assert!(zn.is_group_abelian(Multiplicative));
    //assert!(zn.is_field());

    let x = zn.new(-20);
    println!("x = {}", -20);

    let y = zn.new(25);
    println!("y = {}", 25);
    
    let z = x.op(Multiplicative, &y);
    println!("x*y mod 2^8 = {}", z.0);
}
