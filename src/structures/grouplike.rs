
//! TODO: 
//! * expand on why we have parent/element structure (see Parent doc)
//! * explain use of A: Magma<Element=<Self as A>::Element>
//! * explain A::Parent: Element<Parent=Self> (same idea)
//! * comment on blanket impls
//! * move comments describing properties to docs in properties.rs

use crate::*;
use crate::properties::*;


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
pub trait Magma<O: Operator>: 
    Parent<Element=<Self as Magma<O>>::Element> 
{
    type Element: MagmaElement<O, Parent=Self>;
    fn is_magma(&self, _: O) -> bool { true }
}

pub trait MagmaElement<O: Operator>:
    Element<Parent=<Self as MagmaElement<O>>::Parent> 
{
    type Parent: Magma<O, Element=Self>;

    /// Performs an operation.
    fn operate(&self, right: &Self) -> Self;

    /// Performs specific operation.
    #[inline]
    fn op(&self, _: O, lhs: &Self) -> Self {
        self.operate(lhs)
    }
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
pub trait Quasigroup<O: Operator>: 
    Magma<O, Element=<Self as Quasigroup<O>>::Element>
    + Divisible<O>
{
    type Element: QuasigroupElement<O, Parent=Self>;
    fn is_quasigroup(&self, _: O) -> bool { true }
}

pub trait QuasigroupElement<O: Operator>:
    MagmaElement<O, Parent=<Self as QuasigroupElement<O>>::Parent>
    + TwoSidedInverse<O>
{
    type Parent: Quasigroup<O, Element=Self>;
}

impl<T, O> Quasigroup<O> for T
where
    O: Operator,
    T: Magma<O> 
    + Divisible<O>,
    <T as Magma<O>>::Element: TwoSidedInverse<O>
{
    type Element = <T as Magma<O>>::Element;
}

impl<T, O> QuasigroupElement<O> for T
where
    O: Operator,
    T: MagmaElement<O>
    + TwoSidedInverse<O>,
    <T as MagmaElement<O>>::Parent: Quasigroup<O, Element=Self>
{
    type Parent = <T as MagmaElement<O>>::Parent;
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
pub trait Semigroup<O: Operator>: 
    Magma<O, Element=<Self as Semigroup<O>>::Element> 
    + Associative<O>
{
    type Element: SemigroupElement<O, Parent=Self>;
    fn is_semigroup(&self, _: O) -> bool { true }
}

pub trait SemigroupElement<O: Operator>: 
    MagmaElement<O, Parent=<Self as SemigroupElement<O>>::Parent>
{
    type Parent: Semigroup<O, Element=Self>;
}

impl<T, O> Semigroup<O> for T
where
    O: Operator,
    T: Magma<O> 
    + Associative<O>,
{
    type Element = <T as Magma<O>>::Element;
}

impl<T, O> SemigroupElement<O> for T
where
    O: Operator,
    T: MagmaElement<O>,
    <T as MagmaElement<O>>::Parent: Semigroup<O, Element=Self>
{
    type Parent = <T as MagmaElement<O>>::Parent;
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
pub trait Loop<O: Operator>: 
    Quasigroup<O, Element=<Self as Loop<O>>::Element> 
    + Identity<O> 
{
    type Element: LoopElement<O, Parent=Self>;
    fn is_loop(&self, _: O) -> bool { true }
}

pub trait LoopElement<O: Operator>: 
    QuasigroupElement<O, Parent=<Self as LoopElement<O>>::Parent>
    + IsIdentity<O> 
{
    type Parent: Loop<O, Element=Self>;

}

impl<T, O> Loop<O> for T
where
    O: Operator,
    T: Quasigroup<O> 
    + Identity<O>,
    <T as Quasigroup<O>>::Element: IsIdentity<O>
{
    type Element = <T as Magma<O>>::Element;
}

impl<T, O> LoopElement<O> for T
where
    O: Operator,
    T: QuasigroupElement<O>
    + IsIdentity<O>,
    <T as QuasigroupElement<O>>::Parent: Loop<O, Element=Self>
{
    type Parent = <T as MagmaElement<O>>::Parent;
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
pub trait Monoid<O: Operator>: 
    Semigroup<O, Element=<Self as Monoid<O>>::Element> 
    + Identity<O> 
{
    type Element: MonoidElement<O, Parent=Self>;
    fn is_monoid(&self, _: O) -> bool { true }
}

pub trait MonoidElement<O: Operator>: 
    SemigroupElement<O, Parent=<Self as MonoidElement<O>>::Parent> 
    + IsIdentity<O> 
{
    type Parent: Monoid<O, Element=Self>;
}

impl<T, O> Monoid<O> for T
where
    O: Operator,
    T: Semigroup<O> 
    + Identity<O>,
    <T as Semigroup<O>>::Element: IsIdentity<O>
    //<T as Semigroup<O>>::Element: MonoidElement<O, Parent=Self>
{
    type Element = <T as Magma<O>>::Element;
}

impl<T, O> MonoidElement<O> for T
where
    O: Operator,
    T: SemigroupElement<O>
    + IsIdentity<O>,
    <T as SemigroupElement<O>>::Parent: Monoid<O, Element=Self>
{
    type Parent = <T as MagmaElement<O>>::Parent;
}

/// A group is a loop and a monoid  at the same time.
///
/// *A groups is a set with a closed associative binary operation with the divisibility property and an identity element.*
pub trait Group<O: Operator>: 
    Loop<O, Element=<Self as Group<O>>::Element> 
    + Associative<O>
{
    type Element: GroupElement<O, Parent=Self>;
    fn is_group(&self, _: O) -> bool { true }
}

pub trait GroupElement<O: Operator>: 
    LoopElement<O, Parent=<Self as GroupElement<O>>::Parent> 
{
    type Parent: Group<O, Element=Self>;
}

impl<T, O> Group<O> for T
where
    O: Operator,
    T: Loop<O> 
    + Associative<O>,
    //<T as Loop<O>>::Element: GroupElement<O, Parent=Self>
{
    type Element = <T as Magma<O>>::Element;
}

impl<T, O> GroupElement<O> for T
where
    O: Operator,
    T: LoopElement<O>,
    <T as LoopElement<O>>::Parent: Group<O, Element=Self>
{
    type Parent = <T as MagmaElement<O>>::Parent;
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
pub trait GroupAbelian<O: Operator>: 
    Group<O, Element=<Self as GroupAbelian<O>>::Element> 
    + Commutative<O>
{
    type Element: GroupAbelianElement<O, Parent=Self>;
    fn is_group_abelian(&self, _: O) -> bool { true }
}

pub trait GroupAbelianElement<O: Operator>: 
    GroupElement<O, Parent=<Self as GroupAbelianElement<O>>::Parent> 
{
    type Parent: GroupAbelian<O, Element=Self>;
}

impl<T, O> GroupAbelian<O> for T
where
    O: Operator,
    T: Group<O> 
    + Commutative<O>,
    //<T as Group<O>>::Element: GroupAbelianElement<O, Parent=Self>
{
    type Element = <T as Magma<O>>::Element;
}

impl<T, O> GroupAbelianElement<O> for T
where
    O: Operator,
    T: GroupElement<O>,
    <T as GroupElement<O>>::Parent: GroupAbelian<O, Element=Self>
{
    type Parent = <T as MagmaElement<O>>::Parent;
}

// Implementations

