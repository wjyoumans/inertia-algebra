use crate::*;
use crate::structures::*;
use crate::properties::*;

/// A **ring** is the combination of an Abelian group and a multiplicative monoid 
/// structure.
///
/// A ring is equipped with:
///
/// * An abstract operator (usually the addition, "+") that fulfills the constraints 
/// of an Abelian group.
///
///     *An Abelian group is a set with a closed commutative and associative addition 
///     with the divisibility property and an identity element.*
/// * A second abstract operator (usually the multiplication, "×") that fulfills the 
/// constraints of a monoid.
///
///     *A set equipped with a closed associative multiplication with the divisibility 
///     property and an identity element.*
///
/// The multiplication is distributive over the addition:
///
/// # Distributivity
///
/// ~~~notrust
/// a, b, c ∈ Self, a × (b + c) = a × b + a × c.
/// ~~~
pub trait NCRing<A: Operator = Additive, M: Operator = Multiplicative>:
    GroupAbelian<A, Element=<Self as NCRing<A, M>>::Element> 
    + Monoid<M, Element=<Self as NCRing<A, M>>::Element>
    + Distributive<A, M>
{
    type Element: NCRingElement<A, M, Parent=Self>;
    fn is_ncring(&self, _: A, _: M) -> bool { true }
}

pub trait NCRingElement<A: Operator = Additive, M: Operator = Multiplicative>:
    GroupAbelianElement<A, Parent=<Self as NCRingElement<A, M>>::Parent> 
    + MonoidElement<M, Parent=<Self as NCRingElement<A, M>>::Parent>
{
    type Parent: NCRing<A, M, Element=Self>;
}

impl<Par, Elem, A: Operator, M: Operator> NCRing<A, M> for Par
where
    Par: GroupAbelian<A, Element=Elem>
    + Monoid<M, Element=Elem>
    + Distributive<A, M>,
    Elem: GroupAbelianElement<A, Parent=Par>
    + MonoidElement<M, Parent=Par>
{
    type Element = Elem;
}

impl<Par, Elem, A: Operator, M: Operator> NCRingElement<A, M> for Elem
where
    Elem: GroupAbelianElement<A, Parent=Par>
    + MonoidElement<M, Parent=Par>,
    Par: GroupAbelian<A, Element=Elem>
    + Monoid<M, Element=Elem>
    + Distributive<A, M>
{
    type Parent = Par;
}

/// A ring with a commutative multiplication.
///
/// *A **commutative ring** is a set with two binary operations: a closed commutative and associative with the divisibility property and an identity element,
/// and another closed associative and **commutative** with the divisibility property and an identity element.*
///
/// # Commutativity
///
/// ```notrust
/// ∀ a, b ∈ Self, a × b = b × a
/// ```
pub trait Ring<A: Operator = Additive, M: Operator = Multiplicative>:
    NCRing<A, M, Element=<Self as Ring<A, M>>::Element>
    + Commutative<M>
{
    type Element: RingElement<A, M, Parent=Self>;
    fn is_ring(&self, _: A, _: M) -> bool { true }
}

pub trait RingElement<A: Operator = Additive, M: Operator = Multiplicative>:
    NCRingElement<A, M, Parent=<Self as RingElement<A, M>>::Parent>
{
    type Parent: Ring<A, M, Element=Self>;
}

impl<Par, Elem, A: Operator, M: Operator> Ring<A, M> for Par
where
    Par: NCRing<A, M, Element=Elem> + Commutative<M>,
    Elem: NCRingElement<A, M, Parent=Par>
{
    type Element = Elem;
}

impl<Par, Elem, A: Operator, M: Operator> RingElement<A, M> for Elem
where
    Elem: NCRingElement<A, M, Parent=Par>,
    Par: NCRing<A, M, Element=Elem> + Commutative<M>
{
    type Parent = Par;
}

/// A field is a commutative ring, and an Abelian group under both operators.
///
/// *A **field** is a set with two binary operations, an addition and a multiplication, which are both closed, commutative, associative
/// possess the divisibility property and an identity element, noted 0 and 1 respectively. Furthermore the multiplication is distributive
/// over the addition.*
pub trait Field<A: Operator = Additive, M: Operator = Multiplicative>:
    Ring<A, M, Element=<Self as Field<A, M>>::Element>
    + Divisible<M>
{
    type Element: FieldElement<A, M, Parent=Self>;
    fn is_field(&self, _: A, _: M) -> bool { true }
}

pub trait FieldElement<A: Operator = Additive, M: Operator = Multiplicative>:
    RingElement<A, M, Parent=<Self as FieldElement<A, M>>::Parent> 
    + TwoSidedInverse<M>
{
    type Parent: Field<A, M, Element=Self>;
}

impl<Par, Elem, A: Operator, M: Operator> Field<A, M> for Par
where
    Par: Ring<A, M, Element=Elem> 
    + Divisible<M>,
    Elem: RingElement<A, M, Parent=Par> 
    + TwoSidedInverse<M>
{
    type Element = Elem;
}

impl<Par, Elem, A: Operator, M: Operator> FieldElement<A, M> for Elem
where
    Elem: RingElement<A, M, Parent=Par> 
    + TwoSidedInverse<M>,
    Par: Ring<A, M, Element=Elem>
    + Divisible<M>
{
    type Parent = Par;
}
