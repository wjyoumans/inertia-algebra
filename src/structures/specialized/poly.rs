
use crate::*;
use std::ops::{Deref, DerefMut};

pub trait PolynomialRing<T: Ring>:
    Ring<Element=<Self as PolynomialRing<T>>::Element>
{
    type Element: PolynomialRingElement<T, Parent=Self>;

    /// Initialize the `PolynomialRing` from a base ring and a `String` representing
    /// the variable used for printing.
    fn init<S: Into<String>>(ring: &T, var: S) -> Self;

    /// Return a reference to the base ring.
    fn base_ring(&self) -> &T;

    /// Return the variable as a `String`.
    fn var(&self) -> String;

    fn set_var<S: Into<String>>(&mut self, var: S);

    #[inline]
    fn nvars(&self) -> i64 {
        1
    }

    #[inline]
    fn is_generic(&self) -> bool {
        false
    }
}

pub trait PolynomialRingElement<T: Ring>:
    RingElement<Parent=<Self as PolynomialRingElement<T>>::Parent>
{
    type Parent: PolynomialRing<T, Element=Self>;
    
    type Borrow<'a>: Deref<Target=Elem<T>> where T: 'a;
    type BorrowMut<'a>: DerefMut<Target=Elem<T>> where T: 'a;
    
    //fn parent(&self) -> <Self as PolynomialRingElement<T>>::Parent;

    /// Return a reference to the base ring.
    fn base_ring(&self) -> &T;

    /// Return the variable as a `String`.
    fn var(&self) -> String;

    /// Return the degree of the polynomial, which is always the length - 1.
    #[inline]
    fn degree(&self) -> i64 {
        (self.len() - 1).try_into().unwrap()
    }

    /// Return the length of the polynomial, which is always the degree + 1.
    fn len(&self) -> usize;

    fn get_coefficient(&self, i: usize) -> Elem<T>;
    
    #[inline]
    fn get_coeff(&self, i: usize) -> Elem<T> {
        self.get_coefficient(i)
    }

    fn set_coefficient(&mut self, i: usize, coeff: Elem<T>);
    
    #[inline]
    fn set_coeff(&mut self, i: usize, coeff: Elem<T>) {
        self.set_coefficient(i, coeff);
    }

    fn get_coefficients(&self) -> Vec<Elem<T>>;
    
    #[inline]
    fn get_coeffs(&self) -> Vec<Elem<T>> {
        self.get_coefficients()
    }

    #[inline]
    fn is_generic(&self) -> bool {
        false
    }
}
