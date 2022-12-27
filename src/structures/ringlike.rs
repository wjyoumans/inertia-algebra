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
pub trait AbstractNCRing<A: Operator = Additive, M: Operator = Multiplicative>:
    AbstractGroupAbelian<A, Element=<Self as AbstractNCRing<A, M>>::Element> 
    + AbstractMonoid<M, Element=<Self as AbstractNCRing<A, M>>::Element>
    + Distributive<A, M>
{
    type Element: AbstractNCRingElement<A, M, Parent=Self>;
    fn is_abstract_ncring(&self, _: A, _: M) -> bool { true }
}

pub trait AbstractNCRingElement<A: Operator = Additive, M: Operator = Multiplicative>:
    AbstractGroupAbelianElement<A, Parent=<Self as AbstractNCRingElement<A, M>>::Parent> 
    + AbstractMonoidElement<M, Parent=<Self as AbstractNCRingElement<A, M>>::Parent>
{
    type Parent: AbstractNCRing<A, M, Element=Self>;
}

impl<Par, Elem, A: Operator, M: Operator> AbstractNCRing<A, M> for Par
where
    Par: AbstractGroupAbelian<A, Element=Elem>
    + AbstractMonoid<M, Element=Elem>
    + Distributive<A, M>,
    Elem: AbstractGroupAbelianElement<A, Parent=Par>
    + AbstractMonoidElement<M, Parent=Par>
{
    type Element = Elem;
}

impl<Par, Elem, A: Operator, M: Operator> AbstractNCRingElement<A, M> for Elem
where
    Elem: AbstractGroupAbelianElement<A, Parent=Par>
    + AbstractMonoidElement<M, Parent=Par>,
    Par: AbstractGroupAbelian<A, Element=Elem>
    + AbstractMonoid<M, Element=Elem>
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
pub trait AbstractRing<A: Operator = Additive, M: Operator = Multiplicative>:
    AbstractNCRing<A, M, Element=<Self as AbstractRing<A, M>>::Element>
    + Commutative<M>
{
    type Element: AbstractRingElement<A, M, Parent=Self>;
    fn is_abstract_ring(&self, _: A, _: M) -> bool { true }
}

pub trait AbstractRingElement<A: Operator = Additive, M: Operator = Multiplicative>:
    AbstractNCRingElement<A, M, Parent=<Self as AbstractRingElement<A, M>>::Parent>
{
    type Parent: AbstractRing<A, M, Element=Self>;
}

impl<Par, Elem, A: Operator, M: Operator> AbstractRing<A, M> for Par
where
    Par: AbstractNCRing<A, M, Element=Elem> + Commutative<M>,
    Elem: AbstractNCRingElement<A, M, Parent=Par>
{
    type Element = Elem;
}

impl<Par, Elem, A: Operator, M: Operator> AbstractRingElement<A, M> for Elem
where
    Elem: AbstractNCRingElement<A, M, Parent=Par>,
    Par: AbstractNCRing<A, M, Element=Elem> + Commutative<M>
{
    type Parent = Par;
}

/// A field is a commutative ring, and an Abelian group under both operators.
///
/// *A **field** is a set with two binary operations, an addition and a multiplication, which are both closed, commutative, associative
/// possess the divisibility property and an identity element, noted 0 and 1 respectively. Furthermore the multiplication is distributive
/// over the addition.*
pub trait AbstractField<A: Operator = Additive, M: Operator = Multiplicative>:
    AbstractRing<A, M, Element=<Self as AbstractField<A, M>>::Element>
    + Divisible<M>
{
    type Element: AbstractFieldElement<A, M, Parent=Self>;
    fn is_abstract_field(&self, _: A, _: M) -> bool { true }
}

pub trait AbstractFieldElement<A: Operator = Additive, M: Operator = Multiplicative>:
    AbstractRingElement<A, M, Parent=<Self as AbstractFieldElement<A, M>>::Parent> 
    + TwoSidedInverse<M>
{
    type Parent: AbstractField<A, M, Element=Self>;
}

impl<Par, Elem, A: Operator, M: Operator> AbstractField<A, M> for Par
where
    Par: AbstractRing<A, M, Element=Elem> 
    + Divisible<M>,
    Elem: AbstractRingElement<A, M, Parent=Par> 
    + TwoSidedInverse<M>
{
    type Element = Elem;
}

impl<Par, Elem, A: Operator, M: Operator> AbstractFieldElement<A, M> for Elem
where
    Elem: AbstractRingElement<A, M, Parent=Par> 
    + TwoSidedInverse<M>,
    Par: AbstractRing<A, M, Element=Elem>
    + Divisible<M>
{
    type Parent = Par;
}
