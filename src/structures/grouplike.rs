
// TODO: 
// * expand on why we have parent/element structure (see Parent doc)
// * explain use of A: AbstractMagma<Element=<Self as A>::Element>
// * explain A::Parent: Element<Parent=Self> (same idea)
// * comment on blanket impls
// * move comments describing properties to docs in properties.rs

use crate::*;
use crate::properties::*;

pub type Elem<T> = <T as Parent>::Element;
pub type Par<T> = <T as Element>::Parent;

pub trait Parent {
    type Element: Element<Parent=Self>;
}

pub trait Element: Clone + PartialEq {
    type Parent: Parent<Element=Self>;
    
    fn parent(&self) -> Self::Parent;
}

/// A magma is an algebraic structure which consists of a set equipped with a 
/// binary operation, ∘, which must be closed.
///
/// # Closed binary operation
///
/// ~~~notrust
/// a, b ∈ Self ⇒ a ∘ b ∈ Self
/// ~~~
pub trait AbstractMagma<O: Operator>: 
    Parent<Element=<Self as AbstractMagma<O>>::Element> 
{
    type Element: AbstractMagmaElement<O, Parent=Self>;
    fn is_abstract_magma(&self, _: O) -> bool { true }
}

pub trait AbstractMagmaElement<O: Operator>:
    Element<Parent=<Self as AbstractMagmaElement<O>>::Parent>
    + Operation<O>
{
    type Parent: AbstractMagma<O, Element=Self>;
}

impl<T, O: Operator> AbstractMagma<O> for T
where
    T: Parent,
    Elem<T>: Operation<O>
{
    type Element = Elem<T>;
}

impl<T, O: Operator> AbstractMagmaElement<O> for T
where
    T: Element + Operation<O>
{
    type Parent = Par<T>;
}

/// A quasigroup is a magma which that has the **divisibility property** (or Latin 
/// square property).
/// *A set with a closed binary operation with the divisibility property.*
///
/// Divisibility is a weak form of right and left invertibility.
///
/// # Divisibility or Latin square property
///
/// ```notrust
/// ∀ a, b ∈ Self, ∃! r, l ∈ Self such that l ∘ a = b and a ∘ r = b
/// ```
///
/// The solution to these equations can be written as
///
/// ```notrust
/// r = a \ b and l = b / a
/// ```
///
/// where "\" and "/" are respectively the **left** and **right** division.
pub trait AbstractQuasigroup<O: Operator>: 
    AbstractMagma<O, Element=<Self as AbstractQuasigroup<O>>::Element>
    + Divisible<O>
{
    type Element: AbstractQuasigroupElement<O, Parent=Self>;
    fn is_abstract_quasigroup(&self, _: O) -> bool { true }
}

pub trait AbstractQuasigroupElement<O: Operator>:
    AbstractMagmaElement<O, Parent=<Self as AbstractQuasigroupElement<O>>::Parent>
    + TwoSidedInverse<O>
{
    type Parent: AbstractQuasigroup<O, Element=Self>;
}

impl<T, O: Operator> AbstractQuasigroup<O> for T
where
    T: AbstractMagma<O> + Divisible<O>,
    Elem<T>: TwoSidedInverse<O>
{
    type Element = Elem<T>;
}

impl<T, O: Operator> AbstractQuasigroupElement<O> for T
where
    T: AbstractMagmaElement<O> + TwoSidedInverse<O>,
    Par<T>: Divisible<O>
{
    type Parent = Par<T>;
}

/// A semigroup is a quasigroup that is **associative**.
///
/// *A semigroup is a set equipped with a closed associative binary operation and that has the divisibility property.*
///
/// # Associativity
///
/// ~~~notrust
/// ∀ a, b, c ∈ Self, (a ∘ b) ∘ c = a ∘ (b ∘ c)
/// ~~~
pub trait AbstractSemigroup<O: Operator>: 
    AbstractMagma<O, Element=<Self as AbstractSemigroup<O>>::Element> 
    + Associative<O>
{
    type Element: AbstractSemigroupElement<O, Parent=Self>;
    fn is_abstract_semigroup(&self, _: O) -> bool { true }
}

pub trait AbstractSemigroupElement<O: Operator>: 
    AbstractMagmaElement<O, Parent=<Self as AbstractSemigroupElement<O>>::Parent>
{
    type Parent: AbstractSemigroup<O, Element=Self>;
}

impl<T, O: Operator> AbstractSemigroup<O> for T
where
    T: AbstractMagma<O> + Associative<O>,
{
    type Element = Elem<T>;
}

impl<T, O: Operator> AbstractSemigroupElement<O> for T
where
    T: AbstractMagmaElement<O>,
    Par<T>: Associative<O>
{
    type Parent = Par<T>;
}

/// A loop is a quasigroup with an unique **identity element**, e.
///
/// *A set equipped with a closed binary operation possessing the divisibility property
/// and a unique identity element.*
///
/// # Identity element
///
/// ~~~notrust
/// ∃! e ∈ Self, ∀ a ∈ Self, ∃ r, l ∈ Self such that l ∘ a = a ∘ r = e.
/// ~~~
///
/// The left inverse `r` and right inverse `l` are not required to be equal.
///
/// This property follows from
///
/// ~~~notrust
/// ∀ a ∈ Self, ∃ e ∈ Self, such that e ∘ a = a ∘ e = a.
/// ~~~
pub trait AbstractLoop<O: Operator>: 
    AbstractQuasigroup<O, Element=<Self as AbstractLoop<O>>::Element> 
    + Identity<O> 
{
    type Element: AbstractLoopElement<O, Parent=Self>;
    fn is_abstract_loop(&self, _: O) -> bool { true }
}

pub trait AbstractLoopElement<O: Operator>: 
    AbstractQuasigroupElement<O, Parent=<Self as AbstractLoopElement<O>>::Parent>
    + IsIdentity<O> 
{
    type Parent: AbstractLoop<O, Element=Self>;

}

impl<T, O: Operator> AbstractLoop<O> for T
where
    T: AbstractQuasigroup<O> + Identity<O>,
    Elem<T>: IsIdentity<O>
{
    type Element = Elem<T>;
}

impl<T, O: Operator> AbstractLoopElement<O> for T
where
    T: AbstractQuasigroupElement<O> + IsIdentity<O>,
    Par<T>: Identity<O>
{
    type Parent = Par<T>;
}

/// A monoid is a semigroup equipped with an identity element, e.
///
/// *A set equipped with a closed associative binary operation with the divisibility property and
/// an identity element.*
///
/// # Identity element
///
/// ~~~notrust
/// ∃ e ∈ Self, ∀ a ∈ Self, e ∘ a = a ∘ e = a
/// ~~~
pub trait AbstractMonoid<O: Operator>: 
    AbstractSemigroup<O, Element=<Self as AbstractMonoid<O>>::Element> 
    + Identity<O> 
{
    type Element: AbstractMonoidElement<O, Parent=Self>;
    fn is_abstract_monoid(&self, _: O) -> bool { true }
}

pub trait AbstractMonoidElement<O: Operator>: 
    AbstractSemigroupElement<O, Parent=<Self as AbstractMonoidElement<O>>::Parent> 
    + IsIdentity<O> 
{
    type Parent: AbstractMonoid<O, Element=Self>;
}

impl<T, O: Operator> AbstractMonoid<O> for T
where
    T: AbstractSemigroup<O> + Identity<O>,
    Elem<T>: IsIdentity<O>
{
    type Element = Elem<T>;
}

impl<T, O: Operator> AbstractMonoidElement<O> for T
where
    T: AbstractSemigroupElement<O> + IsIdentity<O>,
    Par<T>: Identity<O>
{
    type Parent = Par<T>;
}

/// A group is a loop and a monoid  at the same time.
///
/// *A groups is a set with a closed associative binary operation with the divisibility property and an identity element.*
pub trait AbstractGroup<O: Operator>: 
    AbstractLoop<O, Element=<Self as AbstractGroup<O>>::Element> 
    + Associative<O>
{
    type Element: AbstractGroupElement<O, Parent=Self>;
    fn is_abstract_group(&self, _: O) -> bool { true }
}

pub trait AbstractGroupElement<O: Operator>: 
    AbstractLoopElement<O, Parent=<Self as AbstractGroupElement<O>>::Parent> 
{
    type Parent: AbstractGroup<O, Element=Self>;
}

impl<T, O: Operator> AbstractGroup<O> for T
where
    T: AbstractLoop<O> + Associative<O>,
{
    type Element = Elem<T>;
}

impl<T, O: Operator> AbstractGroupElement<O> for T
where
    T: AbstractLoopElement<O>,
    Par<T>: Associative<O>
{
    type Parent = Par<T>;
}

/// An Abelian group is a **commutative** group.
///
/// *An commutative group is a set with a closed commutative and associative binary operation with the divisibility property and an identity element.*
///
/// # Commutativity
///
/// ```notrust
/// ∀ a, b ∈ Self, a ∘ b = b ∘ a
/// ```
pub trait AbstractGroupAbelian<O: Operator>: 
    AbstractGroup<O, Element=<Self as AbstractGroupAbelian<O>>::Element> 
    + Commutative<O>
{
    type Element: AbstractGroupAbelianElement<O, Parent=Self>;
    fn is_abstract_group_abelian(&self, _: O) -> bool { true }
}

pub trait AbstractGroupAbelianElement<O: Operator>: 
    AbstractGroupElement<O, Parent=<Self as AbstractGroupAbelianElement<O>>::Parent> 
{
    type Parent: AbstractGroupAbelian<O, Element=Self>;
}

impl<T, O: Operator> AbstractGroupAbelian<O> for T
where
    T: AbstractGroup<O> + Commutative<O>,
{
    type Element = Elem<T>;
}

impl<T, O: Operator> AbstractGroupAbelianElement<O> for T
where
    T: AbstractGroupElement<O>,
    Par<T>: Commutative<O>
{
    type Parent = Par<T>;
}
