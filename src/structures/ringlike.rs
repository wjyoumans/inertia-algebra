use crate::*;
use crate::structures::*;
use crate::properties::*;

/*
// needed to avoid cycle with NCRing: Distributive, Distributive: NCRing
pub trait DualMagma<A: Operator = Additive, M: Operator = Multiplicative>:
    Parent<Element=<Self as DualMagma<A, M>>::Element>
    + Magma<A, Element=<Self as DualMagma<A, M>>::Element> 
    + Magma<M, Element=<Self as DualMagma<A, M>>::Element> 
{
    type Element: DualMagmaElement<A, M, Parent=Self>;
    fn is_dualmagma(&self) -> bool { true }
}

pub trait DualMagmaElement<A: Operator = Additive, M: Operator = Multiplicative>:
    Element<Parent=<Self as DualMagmaElement<A, M>>::Parent>
    + MagmaElement<A, Parent=<Self as DualMagmaElement<A, M>>::Parent> 
    + MagmaElement<M, Parent=<Self as DualMagmaElement<A, M>>::Parent> 
{
    type Parent: DualMagma<A, M, Element=Self>;
}

impl<T, A, M> DualMagma<A, M> for T
where
    A: Operator,
    M: Operator,
    T: Parent,
    + Magma<A, Element=<T as Parent>::Element>
    + Magma<M, Element=<T as Parent>::Element>,
    <T as Parent>::Element: MagmaElement<A, Parent=T>
    + MagmaElement<M, Parent=T>
{
    type Element = <T as Parent>::Element;
}

impl<T, A, M> DualMagmaElement<A, M> for T
where
    A: Operator,
    M: Operator,
    T: Element,
    //+ MagmaElement<A, Parent=<T as Element>::Parent>
    //+ MagmaElement<M, Parent=<T as Element>::Parent>,
    <T as Element>::Parent: Magma<A, Element=T>
    + Magma<M, Element=T>
{
    type Parent = <T as Element>::Parent;
}
*/

/*
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
    Parent<Element=<Self as NCRing<A, M>>::Element>
    //DualMagma<Element=<Self as NCRing<A, M>>::Element>
    + GroupAbelian<A, Element=<Self as NCRing<A, M>>::Element> 
    + Monoid<M, Element=<Self as NCRing<A, M>>::Element>
    //+ Distributive<A, M>
{
    type Element: NCRingElement<A, M, Parent=Self>;
    fn is_ncring(&self) -> bool { true }
}

pub trait NCRingElement<A: Operator = Additive, M: Operator = Multiplicative>:
    Element<Parent=<Self as NCRingElement<A, M>>::Parent>
    //DualMagmaElement<Parent=<Self as NCRingElement<A, M>>::Parent>
    + GroupAbelianElement<A, Parent=<Self as NCRingElement<A, M>>::Parent> 
    + MonoidElement<M, Parent=<Self as NCRingElement<A, M>>::Parent>
{
    type Parent: NCRing<A, M, Element=Self>;
}
*/

/*
impl<T, A, M> NCRing<A, M> for T
where
    A: Operator,
    M: Operator,
    T: Parent
    + GroupAbelian<A, Element=<T as Parent>::Element>
    + Monoid<M, Element=<T as Parent>::Element>,
    //+ Distributive<A, M>,
    //<T as Parent>::Element: NCRingElement<A, M, Parent=T>
    <T as Parent>::Element: GroupAbelianElement<A, Parent=T>
    + MonoidElement<M, Parent=T>
{
    type Element = <T as Parent>::Element;
}

impl<T, A, M> NCRingElement<A, M> for T
where
    A: Operator,
    M: Operator,
    T: Element
    + GroupAbelianElement<A, Parent=<T as Element>::Parent>
    + MonoidElement<M, Parent=<T as Element>::Parent>,
    //<T as Element>::Parent: NCRing<A, M, Element=Self>
    <T as Element>::Parent: GroupAbelian<A, Element=T>
    + Monoid<M, Element=T>

{
    type Parent = <T as Element>::Parent;
}
*/

/*
pub trait NCRing<A: Operator = Additive, M: Operator = Multiplicative>:
    DualMagma<A, M, Element=<Self as NCRing<A, M>>::Element>
    + GroupAbelian<A, Element=<Self as NCRing<A, M>>::Element> 
    + Monoid<M, Element=<Self as NCRing<A, M>>::Element>
    //+ Distributive<A, M>
{
    type Element: NCRingElement<A, M, Parent=Self>;
    fn is_ncring(&self) -> bool { true }
}

pub trait NCRingElement<A: Operator = Additive, M: Operator = Multiplicative>:
    DualMagmaElement<A, M, Parent=<Self as NCRingElement<A, M>>::Parent>
    + GroupAbelianElement<A, Parent=<Self as NCRingElement<A, M>>::Parent> 
    + MonoidElement<M, Parent=<Self as NCRingElement<A, M>>::Parent>
{
    type Parent: NCRing<A, M, Element=Self>;
}

impl<T, A, M> NCRing<A, M> for T
where
    A: Operator,
    M: Operator,
    T: DualMagma<A, M> 
    + GroupAbelian<A, Element=<T as DualMagma<A, M>>::Element>
    + Monoid<M, Element=<T as DualMagma<A, M>>::Element>,
    //+ Distributive<A, M>,
    <T as DualMagma<A, M>>::Element: TwoSidedInverse<A> 
    + IsIdentity<A>
    + IsIdentity<M>
{
    type Element = <T as DualMagma<A, M>>::Element;
}

impl<T, A, M> NCRingElement<A, M> for T
where
    A: Operator,
    M: Operator,
    T: DualMagmaElement<A, M> 
    + GroupAbelianElement<A, Parent=<T as DualMagmaElement<A, M>>::Parent>
    + MonoidElement<M, Parent=<T as DualMagmaElement<A, M>>::Parent>,
    <T as DualMagmaElement<A, M>>::Parent: NCRing<A, M, Element=Self>
{
    type Parent = <T as DualMagmaElement<A, M>>::Parent;
}
*/

/*
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
    fn is_ring(&self) -> bool { true }
}

pub trait RingElement<A: Operator = Additive, M: Operator = Multiplicative>:
    NCRingElement<A, M, Parent=<Self as RingElement<A, M>>::Parent>
{
    type Parent: Ring<A, M, Element=Self>;
}
impl<T, A, M> Ring<A, M> for T
where
    A: Operator,
    M: Operator,
    T: NCRing<A, M> 
    + Commutative<M>,
    //<T as NCRing<A, M>>::Element: RingElement<A, M, Parent=Self>
{
    type Element = <T as DualMagma<A, M>>::Element;
}

impl<T, A, M> RingElement<A, M> for T
where
    A: Operator,
    M: Operator,
    T: NCRingElement<A, M>,
    <T as NCRingElement<A, M>>::Parent: Ring<A, M, Element=Self>
{
    type Parent = <T as DualMagmaElement<A, M>>::Parent;
}

/*
pub trait Ring<A: Operator = Additive, M: Operator = Multiplicative>:
    NCRing<A, M, Element=<Self as Ring<A, M>>::Element>
    + Commutative<M>
{
    type Element: RingElement<A, M, Parent=Self>;
    fn is_ring(&self) -> bool { true }
}

pub trait RingElement<A: Operator = Additive, M: Operator = Multiplicative>:
    NCRingElement<A, M, Parent=<Self as RingElement<A, M>>::Parent>
{
    type Parent: Ring<A, M, Element=Self>;
}
*/

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
    fn is_field(&self) -> bool { true }
}

pub trait FieldElement<A: Operator = Additive, M: Operator = Multiplicative>:
    RingElement<A, M, Parent=<Self as FieldElement<A, M>>::Parent> 
    + TwoSidedInverse<M>
{
    type Parent: Field<A, M, Element=Self>;
}

impl<T, A, M> Field<A, M> for T
where
    A: Operator,
    M: Operator,
    T: Ring<A, M> 
    + Divisible<M>,
    <T as Ring<A, M>>::Element: TwoSidedInverse<M>
    //<T as Ring<A, M>>::Element: FieldElement<A, M, Parent=Self>
{
    type Element = <T as DualMagma<A, M>>::Element;
}

impl<T, A, M> FieldElement<A, M> for T
where
    A: Operator,
    M: Operator,
    T: RingElement<A, M>
    + TwoSidedInverse<M>,
    <T as RingElement<A, M>>::Parent: Field<A, M, Element=Self>
{
    type Parent = <T as DualMagmaElement<A, M>>::Parent;
}
*/
