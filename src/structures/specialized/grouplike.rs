
use crate::*;
use crate::ops::*;
use crate::structures::*;

/* Do macro stuff
macro_rules! specialize_structures(
    // **With type parameters** for the trait being implemented.
    (
        $specialized_parent:ident, 
        $specialized_element:ident, 
        $abstract_parent_trait:ident<$($ops: ident),*> : $($bounds: ident)*
        $abstract_element_trait:ident<$($ops: ident),*> : $($bounds: ident)*
    ) => {
        /// [Alias] Algebraic structure specialized for one kind of operation.
        pub trait $specialized: $abstract_trait<$($ops),*> $(+ $bounds)* { }
        impl<T: $abstract_trait<$($ops),*> $(+ $bounds)*> $specialized for T { }
    };
    // **Without type parameters** for the trait being implemented.
    (
        $specialized_parent:ident, 
        $specialized_element:ident, 
        $abstract_trait: ident : $($bounds: ident)*
    ) => {
        /// [Alias] Algebraic structure specialized for one kind of operation.
        pub trait $specialized: $abstract_trait $(+ $bounds)* { }
        impl<T: $abstract_trait $(+ $bounds)*> $specialized for T { }
    }
);

specialize_structures! {
    AdditiveMagma,
    AbstractMagma<Additive>: 
}
specialize_structures! {
    AdditiveQuasigroup, 
    AbstractQuasigroup<Additive>: AdditiveMagma SubOps
}
*/

// ADDITIVE

// Magma

pub trait AddMagmaOps: AddOps {}

impl<T> AddMagmaOps for T where T: AddOps {}

pub trait AdditiveMagma:
    AbstractMagma<Additive, Element=<Self as AdditiveMagma>::Element> 
{
    type Element: AdditiveMagmaElement<Parent=Self>;
    fn is_additive_magma(&self) -> bool { true }
}

pub trait AdditiveMagmaElement:
    AbstractMagmaElement<Additive, Parent=<Self as AdditiveMagmaElement>::Parent>
    + AddMagmaOps
{
    type Parent: AdditiveMagma<Element=Self>;
}

impl<T> AdditiveMagma for T
where
    T: AbstractMagma<Additive>,
    <T as AbstractMagma<Additive>>::Element: AddMagmaOps
{
    type Element = Elem<T>;
}

impl<T> AdditiveMagmaElement for T
where
    T: AbstractMagmaElement<Additive> + AddMagmaOps
{
    type Parent = Par<T>;
}

// Quasigroup

pub trait AddQuasigroupOps: AddOps + NegOps + SubOps {}

impl<T> AddQuasigroupOps for T where T: AddOps + NegOps + SubOps {}

pub trait AdditiveQuasigroup: 
    AbstractQuasigroup<Additive, Element=<Self as AdditiveQuasigroup>::Element>
{
    type Element: AdditiveQuasigroupElement<Parent=Self>;
    fn is_additive_quasigroup(&self) -> bool { true }
}

pub trait AdditiveQuasigroupElement:
    AbstractQuasigroupElement<Additive, Parent=<Self as AdditiveQuasigroupElement>::Parent>
    + AddQuasigroupOps
{
    type Parent: AdditiveQuasigroup<Element=Self>;
}

impl<T> AdditiveQuasigroup for T
where
    T: AbstractQuasigroup<Additive>,
    <T as AbstractQuasigroup<Additive>>::Element: AddQuasigroupOps 
{
    type Element = Elem<T>;
}

impl<T> AdditiveQuasigroupElement for T
where
    T: AbstractQuasigroupElement<Additive> + AddQuasigroupOps
{
    type Parent = Par<T>;
}

// Semigroup

pub trait AdditiveSemigroup: 
    AbstractSemigroup<Additive, Element=<Self as AdditiveSemigroup>::Element>
{
    type Element: AdditiveSemigroupElement<Parent=Self>;
    fn is_additive_semigroup(&self) -> bool { true }
}

pub trait AdditiveSemigroupElement:
    AbstractSemigroupElement<Additive, Parent=<Self as AdditiveSemigroupElement>::Parent>
    + AddMagmaOps
{
    type Parent: AdditiveSemigroup<Element=Self>;
}

impl<T> AdditiveSemigroup for T
where
    T: AbstractSemigroup<Additive>,
    <T as AbstractSemigroup<Additive>>::Element: AddMagmaOps
{
    type Element = Elem<T>;
}

impl<T> AdditiveSemigroupElement for T
where
    T: AbstractSemigroupElement<Additive> + AddMagmaOps
{
    type Parent = Par<T>;
}

// Loop

pub trait AdditiveLoop: 
    AbstractLoop<Additive, Element=<Self as AdditiveLoop>::Element>
{
    type Element: AdditiveLoopElement<Parent=Self>;
    fn is_additive_loop(&self) -> bool { true }
}

pub trait AdditiveLoopElement:
    AbstractLoopElement<Additive, Parent=<Self as AdditiveLoopElement>::Parent>
    + AddQuasigroupOps
{
    type Parent: AdditiveLoop<Element=Self>;
}

impl<T> AdditiveLoop for T
where
    T: AbstractLoop<Additive>,
    <T as AbstractLoop<Additive>>::Element: AddQuasigroupOps
{
    type Element = Elem<T>;
}

impl<T> AdditiveLoopElement for T
where
    T: AbstractLoopElement<Additive> + AddQuasigroupOps
{
    type Parent = Par<T>;
}

// Monoid

pub trait AdditiveMonoid: 
    AbstractMonoid<Additive, Element=<Self as AdditiveMonoid>::Element>
{
    type Element: AdditiveMonoidElement<Parent=Self>;
    fn is_additive_monoid(&self) -> bool { true }
}

pub trait AdditiveMonoidElement:
    AbstractMonoidElement<Additive, Parent=<Self as AdditiveMonoidElement>::Parent>
    + AddQuasigroupOps
{
    type Parent: AdditiveMonoid<Element=Self>;
}

impl<T> AdditiveMonoid for T
where
    T: AbstractMonoid<Additive>,
    <T as AbstractMonoid<Additive>>::Element: AddQuasigroupOps
{
    type Element = Elem<T>;
}

impl<T> AdditiveMonoidElement for T
where
    T: AbstractMonoidElement<Additive> + AddQuasigroupOps
{
    type Parent = Par<T>;
}

// Group

pub trait AdditiveGroup: 
    AbstractGroup<Additive, Element=<Self as AdditiveGroup>::Element>
{
    type Element: AdditiveGroupElement<Parent=Self>;
    fn is_additive_group(&self) -> bool { true }
}

pub trait AdditiveGroupElement:
    AbstractGroupElement<Additive, Parent=<Self as AdditiveGroupElement>::Parent>
    + AddQuasigroupOps
{
    type Parent: AdditiveGroup<Element=Self>;
}

impl<T> AdditiveGroup for T
where
    T: AbstractGroup<Additive>,
    <T as AbstractGroup<Additive>>::Element: AddQuasigroupOps
{
    type Element = Elem<T>;
}

impl<T> AdditiveGroupElement for T
where
    T: AbstractGroupElement<Additive> + AddQuasigroupOps
{
    type Parent = Par<T>;
}

// GroupAbelian

pub trait AdditiveGroupAbelian: 
    AbstractGroupAbelian<Additive, Element=<Self as AdditiveGroupAbelian>::Element>
{
    type Element: AdditiveGroupAbelianElement<Parent=Self>;
    fn is_additive_group_abelian(&self) -> bool { true }
}

pub trait AdditiveGroupAbelianElement:
    AbstractGroupAbelianElement<Additive, Parent=<Self as AdditiveGroupAbelianElement>::Parent>
    + AddQuasigroupOps
{
    type Parent: AdditiveGroupAbelian<Element=Self>;
}

impl<T> AdditiveGroupAbelian for T
where
    T: AbstractGroupAbelian<Additive>,
    <T as AbstractGroupAbelian<Additive>>::Element: AddQuasigroupOps
{
    type Element = Elem<T>;
}

impl<T> AdditiveGroupAbelianElement for T
where
    T: AbstractGroupAbelianElement<Additive> + AddQuasigroupOps
{
    type Parent = Par<T>;
}

// MULTIPLICATIVE

// Magma

pub trait MultiplicativeMagma: 
    AbstractMagma<Multiplicative, Element=<Self as MultiplicativeMagma>::Element> 
{
    type Element: MultiplicativeMagmaElement<Parent=Self>;
    fn is_multiplicative_magma(&self) -> bool { true }
}

pub trait MultiplicativeMagmaElement:
    AbstractMagmaElement<Multiplicative, Parent=<Self as MultiplicativeMagmaElement>::Parent>
    + MulOps
{
    type Parent: MultiplicativeMagma<Element=Self>;
}

impl<T> MultiplicativeMagma for T
where
    T: AbstractMagma<Multiplicative>,
    <T as AbstractMagma<Multiplicative>>::Element: MulOps
{
    type Element = Elem<T>;
}

impl<T> MultiplicativeMagmaElement for T
where
    T: AbstractMagmaElement<Multiplicative>
    + MulOps
{
    type Parent = Par<T>;
}

// Quasigroup

pub trait MultiplicativeQuasigroup: 
    AbstractQuasigroup<Multiplicative, Element=<Self as MultiplicativeQuasigroup>::Element>
{
    type Element: MultiplicativeQuasigroupElement<Parent=Self>;
    fn is_multiplicative_quasigroup(&self) -> bool { true }
}

pub trait MultiplicativeQuasigroupElement:
    AbstractQuasigroupElement<Multiplicative, Parent=<Self as MultiplicativeQuasigroupElement>::Parent>
    + DivOps + InvOps
{
    type Parent: MultiplicativeQuasigroup<Element=Self>;
}

impl<T> MultiplicativeQuasigroup for T
where
    T: AbstractQuasigroup<Multiplicative>,
    <T as AbstractMagma<Multiplicative>>::Element: DivOps + InvOps
{
    type Element = Elem<T>;
}

impl<T> MultiplicativeQuasigroupElement for T
where
    T: AbstractQuasigroupElement<Multiplicative>
    + DivOps + InvOps
{
    type Parent = Par<T>;
}

// Semigroup

pub trait MultiplicativeSemigroup: 
    AbstractSemigroup<Multiplicative, Element=<Self as MultiplicativeSemigroup>::Element>
{
    type Element: MultiplicativeSemigroupElement<Parent=Self>;
    fn is_multiplicative_semigroup(&self) -> bool { true }
}

pub trait MultiplicativeSemigroupElement:
    AbstractSemigroupElement<Multiplicative, Parent=<Self as MultiplicativeSemigroupElement>::Parent>
{
    type Parent: MultiplicativeSemigroup<Element=Self>;
}

impl<T> MultiplicativeSemigroup for T
where
    T: AbstractSemigroup<Multiplicative> 
{
    type Element = Elem<T>;
}

impl<T> MultiplicativeSemigroupElement for T
where
    T: AbstractSemigroupElement<Multiplicative>
{
    type Parent = Par<T>;
}

// Loop

pub trait MultiplicativeLoop: 
    AbstractLoop<Multiplicative, Element=<Self as MultiplicativeLoop>::Element>
{
    type Element: MultiplicativeLoopElement<Parent=Self>;
    fn is_multiplicative_loop(&self) -> bool { true }
}

pub trait MultiplicativeLoopElement:
    AbstractLoopElement<Multiplicative, Parent=<Self as MultiplicativeLoopElement>::Parent>
{
    type Parent: MultiplicativeLoop<Element=Self>;
}

impl<T> MultiplicativeLoop for T
where
    T: AbstractLoop<Multiplicative> 
{
    type Element = Elem<T>;
}

impl<T> MultiplicativeLoopElement for T
where
    T: AbstractLoopElement<Multiplicative>
{
    type Parent = Par<T>;
}

// Monoid

pub trait MultiplicativeMonoid: 
    AbstractMonoid<Multiplicative, Element=<Self as MultiplicativeMonoid>::Element>
{
    type Element: MultiplicativeMonoidElement<Parent=Self>;
    fn is_multiplicative_monoid(&self) -> bool { true }
}

pub trait MultiplicativeMonoidElement:
    AbstractMonoidElement<Multiplicative, Parent=<Self as MultiplicativeMonoidElement>::Parent>
{
    type Parent: MultiplicativeMonoid<Element=Self>;
}

impl<T> MultiplicativeMonoid for T
where
    T: AbstractMonoid<Multiplicative> 
{
    type Element = Elem<T>;
}

impl<T> MultiplicativeMonoidElement for T
where
    T: AbstractMonoidElement<Multiplicative>
{
    type Parent = Par<T>;
}

// Group

pub trait MultiplicativeGroup: 
    AbstractGroup<Multiplicative, Element=<Self as MultiplicativeGroup>::Element>
{
    type Element: MultiplicativeGroupElement<Parent=Self>;
    fn is_multiplicative_group(&self) -> bool { true }
}

pub trait MultiplicativeGroupElement:
    AbstractGroupElement<Multiplicative, Parent=<Self as MultiplicativeGroupElement>::Parent>
{
    type Parent: MultiplicativeGroup<Element=Self>;
}

impl<T> MultiplicativeGroup for T
where
    T: AbstractGroup<Multiplicative> 
{
    type Element = Elem<T>;
}

impl<T> MultiplicativeGroupElement for T
where
    T: AbstractGroupElement<Multiplicative>
{
    type Parent = Par<T>;
}

// GroupAbelian

pub trait MultiplicativeGroupAbelian: 
    AbstractGroupAbelian<Multiplicative, Element=<Self as MultiplicativeGroupAbelian>::Element>
{
    type Element: MultiplicativeGroupAbelianElement<Parent=Self>;
    fn is_multiplicative_group_abelian(&self) -> bool { true }
}

pub trait MultiplicativeGroupAbelianElement:
    AbstractGroupAbelianElement<Multiplicative, Parent=<Self as MultiplicativeGroupAbelianElement>::Parent>
{
    type Parent: MultiplicativeGroupAbelian<Element=Self>;
}

impl<T> MultiplicativeGroupAbelian for T
where
    T: AbstractGroupAbelian<Multiplicative> 
{
    type Element = Elem<T>;
}

impl<T> MultiplicativeGroupAbelianElement for T
where
    T: AbstractGroupAbelianElement<Multiplicative>
{
    type Parent = Par<T>;
}
