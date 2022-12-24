
// TODO: 
// * expand on why we have parent/element structure (see Parent doc)
// * explain use of A: Magma<Element=<Self as A>::Element>
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
pub trait Magma<O: Operator>: 
    Parent<Element=<Self as Magma<O>>::Element> 
{
    type Element: MagmaElement<O, Parent=Self>;
    fn is_magma(&self, _: O) -> bool { true }
}

pub trait MagmaElement<O: Operator>:
    Element<Parent=<Self as MagmaElement<O>>::Parent>
    + Operation<O>
{
    type Parent: Magma<O, Element=Self>;
}

impl<T, O: Operator> Magma<O> for T
where
    T: Parent,
    Elem<T>: Operation<O>
{
    type Element = Elem<T>;
}

impl<T, O: Operator> MagmaElement<O> for T
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

impl<T, O: Operator> Quasigroup<O> for T
where
    T: Magma<O> + Divisible<O>,
    Elem<T>: TwoSidedInverse<O>
{
    type Element = Elem<T>;
}

impl<T, O: Operator> QuasigroupElement<O> for T
where
    T: MagmaElement<O> + TwoSidedInverse<O>,
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

impl<T, O: Operator> Semigroup<O> for T
where
    T: Magma<O> + Associative<O>,
{
    type Element = Elem<T>;
}

impl<T, O: Operator> SemigroupElement<O> for T
where
    T: MagmaElement<O>,
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

impl<T, O: Operator> Loop<O> for T
where
    T: Quasigroup<O> + Identity<O>,
    Elem<T>: IsIdentity<O>
{
    type Element = Elem<T>;
}

impl<T, O: Operator> LoopElement<O> for T
where
    T: QuasigroupElement<O> + IsIdentity<O>,
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

impl<T, O: Operator> Monoid<O> for T
where
    T: Semigroup<O> + Identity<O>,
    Elem<T>: IsIdentity<O>
{
    type Element = Elem<T>;
}

impl<T, O: Operator> MonoidElement<O> for T
where
    T: SemigroupElement<O> + IsIdentity<O>,
    Par<T>: Identity<O>
{
    type Parent = Par<T>;
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

impl<T, O: Operator> Group<O> for T
where
    T: Loop<O> + Associative<O>,
{
    type Element = Elem<T>;
}

impl<T, O: Operator> GroupElement<O> for T
where
    T: LoopElement<O>,
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

impl<T, O: Operator> GroupAbelian<O> for T
where
    T: Group<O> + Commutative<O>,
{
    type Element = Elem<T>;
}

impl<T, O: Operator> GroupAbelianElement<O> for T
where
    T: GroupElement<O>,
    Par<T>: Commutative<O>
{
    type Parent = Par<T>;
}
