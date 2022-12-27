
use crate::*;
use crate::structures::*;

use approx::RelativeEq;

/*
type MagmaElem<T, O> = <T as Magma<O>>::Element;

type MagmaElem2<T, O> = (
    <T as Magma<O>>::Element,
    <T as Magma<O>>::Element
);

type MagmaElem3<T, O> = (
    <T as Magma<O>>::Element,
    <T as Magma<O>>::Element,
    <T as Magma<O>>::Element
);
*/

/// A type that is equipped with identity.
pub trait Identity<O: Operator>: AbstractMagma<O> {
    /// The identity element.
    fn identity(&self) -> Elem<Self>;

    /// Specific identity.
    #[inline]
    fn id(&self, _: O) -> Elem<Self>
    {
        self.identity()
    }
    
    /// Checks whether operating with the identity element is a no-op for the given
    /// argument. Approximate equality is used for verifications.
    fn prop_operating_identity_element_is_noop_approx(
        &self, 
        args: (Elem<Self>,)
    ) -> bool
    where
        Elem<Self>: RelativeEq,
    {
        let (a,) = args;
        relative_eq!(a.operate(&self.identity()), a)
            && relative_eq!(self.identity().operate(&a), a)
    }

    /// Checks whether operating with the identity element is a no-op for the given
    /// argument.
    fn prop_operating_identity_element_is_noop(
        &self, 
        args: (Elem<Self>,)
    ) -> bool
    where
        Elem<Self>: Eq,
    {
        let (a,) = args;
        a.operate(&self.identity()) == a && self.identity().operate(&a) == a
    }
}

pub trait Zero: AbstractMagma<Additive> + Identity<Additive> {
    fn zero(&self) -> Elem<Self> {
        self.identity()
    }
}

impl<T> Zero for T
where
    T: AbstractMagma<Additive> + Identity<Additive>
{}

pub trait One: AbstractMagma<Multiplicative> + Identity<Multiplicative> {
    fn one(&self) -> Elem<Self> {
        self.identity()
    }
}

impl<T> One for T
where
    T: AbstractMagma<Multiplicative> + Identity<Multiplicative>
{}


pub trait Associative<O: Operator>: AbstractMagma<O> { 
    /// Returns `true` if associativity holds for the given arguments. Approximate 
    /// equality is used for verifications.
    fn prop_is_associative_approx(args: (Elem<Self>, Elem<Self>, Elem<Self>)) -> bool
    where
        Elem<Self>: RelativeEq,
    {
        let (a, b, c) = args;
        relative_eq!(a.operate(&b).operate(&c), a.operate(&b.operate(&c)))
    }

    /// Returns `true` if associativity holds for the given arguments.
    fn prop_is_associative(args: (Elem<Self>, Elem<Self>, Elem<Self>)) -> bool
    where
        Elem<Self>: Eq,
    {
        let (a, b, c) = args;
        a.operate(&b).operate(&c) == a.operate(&b.operate(&c))
    }
}


pub trait Divisible<O: Operator>: 
    AbstractMagma<O>
where
    Elem<Self>: TwoSidedInverse<O>
{
    /// Returns `true` if latin squareness holds for the given arguments. Approximate
    /// equality is used for verifications.
    ///
    /// ```notrust
    /// a ~= a / b ∘ b && a ~= a ∘ b / b
    /// ```
    fn prop_inv_is_latin_square_approx(args: (Elem<Self>, Elem<Self>)) -> bool
    where
        Elem<Self>: RelativeEq,
    {
        let (a, b) = args;
        relative_eq!(a, a.operate(&b.two_sided_inverse()).operate(&b))
            && relative_eq!(a, a.operate(&b.operate(&b.two_sided_inverse())))

        // TODO: pseudo inverse?
    }

    /// Returns `true` if latin squareness holds for the given arguments.
    ///
    /// ```notrust
    /// a == a / b * b && a == a * b / b
    /// ```
    fn prop_inv_is_latin_square(args: (Elem<Self>, Elem<Self>)) -> bool
    where
        Elem<Self>: Eq,
    {
        let (a, b) = args;
        a == a.operate(&b.two_sided_inverse()).operate(&b)
            && a == a.operate(&b.operate(&b.two_sided_inverse()))

        // TODO: pseudo inverse?
    }
}

pub trait Commutative<O: Operator>: AbstractMagma<O> {
    /// Returns `true` if the operator is commutative for the given argument tuple. 
    /// Approximate equality is used for verifications.
    fn prop_is_commutative_approx(args: (Elem<Self>, Elem<Self>)) -> bool
    where
        Elem<Self>: RelativeEq
    {
        let (a, b) = args;
        relative_eq!(a.operate(&b), b.operate(&a))
    }

    /// Returns `true` if the operator is commutative for the given argument tuple.
    fn prop_is_commutative(args: (Elem<Self>, Elem<Self>)) -> bool
    where
        Elem<Self>: Eq
    {
        let (a, b) = args;
        a.operate(&b) == b.operate(&a)
    }
    
    /* TODO: better to use wrapper?
    /// Returns `true` if the multiplication operator is commutative for the given 
    /// argument tuple. Approximate equality is used for verifications.
    fn prop_mul_is_commutative_approx(args: RingPropArgs<Self, A, M>) -> bool
    where
        <Self as Ring<A, M>>::Element: RelativeEq,
    {
        let (a, b) = args;
        let a = || W::<_, A, M>::new(a.clone());
        let b = || W::<_, A, M>::new(b.clone());

        relative_eq!(a() * b(), b() * a())
    }

    /// Returns `true` if the multiplication operator is commutative for the given 
    /// argument tuple.
    fn prop_mul_is_commutative(args: RingPropArgs<Self, A, M>) -> bool
    where
        <Self as Ring<A, M>>::Element: Eq,
    {
        let (a, b) = args;
        let a = || W::<_, A, M>::new(a.clone());
        let b = || W::<_, A, M>::new(b.clone());

        a() * b() == b() * a()
    }
    */
}
