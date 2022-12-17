
use crate::*;
use crate::wrapper::Wrapper as W;
use crate::structures::*;

use approx::RelativeEq;

/*
type DualMagmaElem<T, A, M> = <T as DualMagma<A, M>>::Element;

type DualMagmaElem3<T, A, M> = (
    <T as DualMagma<A, M>>::Element,
    <T as DualMagma<A, M>>::Element,
    <T as DualMagma<A, M>>::Element
);
*/
//type Elem<T> = <T as Parent>::Element;

pub trait Distributive<A: Operator = Additive, M: Operator = Multiplicative>: {//Parent {
    /*
    /// Returns `true` if the multiplication and addition operators are distributive 
    /// for the given argument tuple. Approximate equality is used for verifications.
    fn prop_mul_and_add_are_distributive_approx(
        args: (Elem<Self>, Elem<Self>, Elem<Self>)
        //args: DualMagmaElem3<Self, A, M>
    ) -> bool
    where
        Elem<Self>: RelativeEq + Sized,
        //DualMagmaElem<Self, A, M>: RelativeEq + Sized,
    {
        let (a, b, c) = args;
        let a = || W::<_, A, M>::new(a.clone());
        let b = || W::<_, A, M>::new(b.clone());
        let c = || W::<_, A, M>::new(c.clone());

        // Left distributivity
        relative_eq!(a() * (b() + c()), a() * b() + a() * c()) &&
        // Right distributivity
        relative_eq!((b() + c()) * a(), b() * a() + c() * a())
    }
    */
    /*
    /// Returns `true` if the multiplication and addition operators are distributive 
    /// for the given argument tuple.
    fn prop_mul_and_add_are_distributive(args: DualMagmaElem3<Self, A, M>) -> bool
    where
        DualMagmaElem<Self, A, M>: Eq + Sized,
    {
        let (a, b, c) = args;
        let a = || W::<_, A, M>::new(a.clone());
        let b = || W::<_, A, M>::new(b.clone());
        let c = || W::<_, A, M>::new(c.clone());

        // Left distributivity
        a() * (b() + c()) == (a() * b()) + (a() * c()) &&
        // Right distributivity
        (b() + c()) * a() == (b() * a()) + (c() * a())
    }
    */

}
