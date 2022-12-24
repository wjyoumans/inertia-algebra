
use crate::*;
use crate::ops::*;
use crate::structures::*;


// ADDITIVE

// Magma

pub trait AdditiveMagma:
    Magma<Additive, Element=<Self as AdditiveMagma>::Element> 
{
    type Element: AdditiveMagmaElement<Parent=Self>;
    fn is_additive_magma(&self) -> bool { true }
}

pub trait AdditiveMagmaElement:
    MagmaElement<Additive, Parent=<Self as AdditiveMagmaElement>::Parent>
    + AddOps
{
    type Parent: AdditiveMagma<Element=Self>;
}

impl<T> AdditiveMagma for T
where
    T: Magma<Additive>,
    <T as Magma<Additive>>::Element: AddOps
{
    type Element = <Self as Magma<Additive>>::Element;
}

impl<T> AdditiveMagmaElement for T
where
    T: MagmaElement<Additive>
    + AddOps
{
    type Parent = <Self as MagmaElement<Additive>>::Parent;
}

// Quasigroup

pub trait AdditiveQuasigroup: 
    Quasigroup<Additive, Element=<Self as AdditiveQuasigroup>::Element>
{
    type Element: AdditiveQuasigroupElement<Parent=Self>;
    fn is_additive_quasigroup(&self) -> bool { true }
}

pub trait AdditiveQuasigroupElement:
    QuasigroupElement<Additive, Parent=<Self as AdditiveQuasigroupElement>::Parent>
    + NegOps
    + SubOps
{
    type Parent: AdditiveQuasigroup<Element=Self>;
}

impl<T> AdditiveQuasigroup for T
where
    T: Quasigroup<Additive> ,
    <T as Quasigroup<Additive>>::Element: NegOps + SubOps
{
    type Element = <T as Quasigroup<Additive>>::Element;
}

impl<T> AdditiveQuasigroupElement for T
where
    T: QuasigroupElement<Additive>
    + NegOps
    + SubOps
{
    type Parent = <T as QuasigroupElement<Additive>>::Parent;
}

// Semigroup

pub trait AdditiveSemigroup: 
    Semigroup<Additive, Element=<Self as AdditiveSemigroup>::Element>
{
    type Element: AdditiveSemigroupElement<Parent=Self>;
    fn is_additive_quasigroup(&self) -> bool { true }
}

pub trait AdditiveSemigroupElement:
    SemigroupElement<Additive, Parent=<Self as AdditiveSemigroupElement>::Parent>
{
    type Parent: AdditiveSemigroup<Element=Self>;
}

impl<T> AdditiveSemigroup for T
where
    T: Semigroup<Additive> 
{
    type Element = <T as Semigroup<Additive>>::Element;
}

impl<T> AdditiveSemigroupElement for T
where
    T: SemigroupElement<Additive>
{
    type Parent = <T as SemigroupElement<Additive>>::Parent;
}

// Loop

pub trait AdditiveLoop: 
    Loop<Additive, Element=<Self as AdditiveLoop>::Element>
{
    type Element: AdditiveLoopElement<Parent=Self>;
    fn is_additive_quasigroup(&self) -> bool { true }
}

pub trait AdditiveLoopElement:
    LoopElement<Additive, Parent=<Self as AdditiveLoopElement>::Parent>
{
    type Parent: AdditiveLoop<Element=Self>;
}

impl<T> AdditiveLoop for T
where
    T: Loop<Additive> 
{
    type Element = <T as Loop<Additive>>::Element;
}

impl<T> AdditiveLoopElement for T
where
    T: LoopElement<Additive>
{
    type Parent = <T as LoopElement<Additive>>::Parent;
}

// Monoid

pub trait AdditiveMonoid: 
    Monoid<Additive, Element=<Self as AdditiveMonoid>::Element>
{
    type Element: AdditiveMonoidElement<Parent=Self>;
    fn is_additive_quasigroup(&self) -> bool { true }
}

pub trait AdditiveMonoidElement:
    MonoidElement<Additive, Parent=<Self as AdditiveMonoidElement>::Parent>
{
    type Parent: AdditiveMonoid<Element=Self>;
}

impl<T> AdditiveMonoid for T
where
    T: Monoid<Additive> 
{
    type Element = <T as Monoid<Additive>>::Element;
}

impl<T> AdditiveMonoidElement for T
where
    T: MonoidElement<Additive>
{
    type Parent = <T as MonoidElement<Additive>>::Parent;
}

// Group

pub trait AdditiveGroup: 
    Group<Additive, Element=<Self as AdditiveGroup>::Element>
{
    type Element: AdditiveGroupElement<Parent=Self>;
    fn is_additive_quasigroup(&self) -> bool { true }
}

pub trait AdditiveGroupElement:
    GroupElement<Additive, Parent=<Self as AdditiveGroupElement>::Parent>
{
    type Parent: AdditiveGroup<Element=Self>;
}

impl<T> AdditiveGroup for T
where
    T: Group<Additive> 
{
    type Element = <T as Group<Additive>>::Element;
}

impl<T> AdditiveGroupElement for T
where
    T: GroupElement<Additive>
{
    type Parent = <T as GroupElement<Additive>>::Parent;
}

// GroupAbelian

pub trait AdditiveGroupAbelian: 
    GroupAbelian<Additive, Element=<Self as AdditiveGroupAbelian>::Element>
{
    type Element: AdditiveGroupAbelianElement<Parent=Self>;
    fn is_additive_quasigroup(&self) -> bool { true }
}

pub trait AdditiveGroupAbelianElement:
    GroupAbelianElement<Additive, Parent=<Self as AdditiveGroupAbelianElement>::Parent>
{
    type Parent: AdditiveGroupAbelian<Element=Self>;
}

impl<T> AdditiveGroupAbelian for T
where
    T: GroupAbelian<Additive> 
{
    type Element = <T as GroupAbelian<Additive>>::Element;
}

impl<T> AdditiveGroupAbelianElement for T
where
    T: GroupAbelianElement<Additive>
{
    type Parent = <T as GroupAbelianElement<Additive>>::Parent;
}

// MULTIPLICATIVE

// Magma

pub trait MultiplicativeMagma: 
    Magma<Multiplicative, Element=<Self as MultiplicativeMagma>::Element> 
{
    type Element: MultiplicativeMagmaElement<Parent=Self>;
    fn is_additive_magma(&self) -> bool { true }
}

pub trait MultiplicativeMagmaElement:
    MagmaElement<Multiplicative, Parent=<Self as MultiplicativeMagmaElement>::Parent>
    + MulOps
{
    type Parent: MultiplicativeMagma<Element=Self>;
}

impl<T> MultiplicativeMagma for T
where
    T: Magma<Multiplicative>,
    <T as Magma<Multiplicative>>::Element: MulOps
{
    type Element = <Self as Magma<Multiplicative>>::Element;
}

impl<T> MultiplicativeMagmaElement for T
where
    T: MagmaElement<Multiplicative>
    + MulOps
{
    type Parent = <Self as MagmaElement<Multiplicative>>::Parent;
}

// Quasigroup

pub trait MultiplicativeQuasigroup: 
    Quasigroup<Multiplicative, Element=<Self as MultiplicativeQuasigroup>::Element>
{
    type Element: MultiplicativeQuasigroupElement<Parent=Self>;
    fn is_additive_quasigroup(&self) -> bool { true }
}

pub trait MultiplicativeQuasigroupElement:
    QuasigroupElement<Multiplicative, Parent=<Self as MultiplicativeQuasigroupElement>::Parent>
    + DivOps + InvOps
{
    type Parent: MultiplicativeQuasigroup<Element=Self>;
}

impl<T> MultiplicativeQuasigroup for T
where
    T: Quasigroup<Multiplicative>,
    <T as Magma<Multiplicative>>::Element: DivOps + InvOps
{
    type Element = <T as Quasigroup<Multiplicative>>::Element;
}

impl<T> MultiplicativeQuasigroupElement for T
where
    T: QuasigroupElement<Multiplicative>
    + DivOps + InvOps
{
    type Parent = <T as QuasigroupElement<Multiplicative>>::Parent;
}

// Semigroup

pub trait MultiplicativeSemigroup: 
    Semigroup<Multiplicative, Element=<Self as MultiplicativeSemigroup>::Element>
{
    type Element: MultiplicativeSemigroupElement<Parent=Self>;
    fn is_additive_quasigroup(&self) -> bool { true }
}

pub trait MultiplicativeSemigroupElement:
    SemigroupElement<Multiplicative, Parent=<Self as MultiplicativeSemigroupElement>::Parent>
{
    type Parent: MultiplicativeSemigroup<Element=Self>;
}

impl<T> MultiplicativeSemigroup for T
where
    T: Semigroup<Multiplicative> 
{
    type Element = <T as Semigroup<Multiplicative>>::Element;
}

impl<T> MultiplicativeSemigroupElement for T
where
    T: SemigroupElement<Multiplicative>
{
    type Parent = <T as SemigroupElement<Multiplicative>>::Parent;
}

// Loop

pub trait MultiplicativeLoop: 
    Loop<Multiplicative, Element=<Self as MultiplicativeLoop>::Element>
{
    type Element: MultiplicativeLoopElement<Parent=Self>;
    fn is_additive_quasigroup(&self) -> bool { true }
}

pub trait MultiplicativeLoopElement:
    LoopElement<Multiplicative, Parent=<Self as MultiplicativeLoopElement>::Parent>
{
    type Parent: MultiplicativeLoop<Element=Self>;
}

impl<T> MultiplicativeLoop for T
where
    T: Loop<Multiplicative> 
{
    type Element = <T as Loop<Multiplicative>>::Element;
}

impl<T> MultiplicativeLoopElement for T
where
    T: LoopElement<Multiplicative>
{
    type Parent = <T as LoopElement<Multiplicative>>::Parent;
}

// Monoid

pub trait MultiplicativeMonoid: 
    Monoid<Multiplicative, Element=<Self as MultiplicativeMonoid>::Element>
{
    type Element: MultiplicativeMonoidElement<Parent=Self>;
    fn is_additive_quasigroup(&self) -> bool { true }
}

pub trait MultiplicativeMonoidElement:
    MonoidElement<Multiplicative, Parent=<Self as MultiplicativeMonoidElement>::Parent>
{
    type Parent: MultiplicativeMonoid<Element=Self>;
}

impl<T> MultiplicativeMonoid for T
where
    T: Monoid<Multiplicative> 
{
    type Element = <T as Monoid<Multiplicative>>::Element;
}

impl<T> MultiplicativeMonoidElement for T
where
    T: MonoidElement<Multiplicative>
{
    type Parent = <T as MonoidElement<Multiplicative>>::Parent;
}

// Group

pub trait MultiplicativeGroup: 
    Group<Multiplicative, Element=<Self as MultiplicativeGroup>::Element>
{
    type Element: MultiplicativeGroupElement<Parent=Self>;
    fn is_additive_quasigroup(&self) -> bool { true }
}

pub trait MultiplicativeGroupElement:
    GroupElement<Multiplicative, Parent=<Self as MultiplicativeGroupElement>::Parent>
{
    type Parent: MultiplicativeGroup<Element=Self>;
}

impl<T> MultiplicativeGroup for T
where
    T: Group<Multiplicative> 
{
    type Element = <T as Group<Multiplicative>>::Element;
}

impl<T> MultiplicativeGroupElement for T
where
    T: GroupElement<Multiplicative>
{
    type Parent = <T as GroupElement<Multiplicative>>::Parent;
}

// GroupAbelian

pub trait MultiplicativeGroupAbelian: 
    GroupAbelian<Multiplicative, Element=<Self as MultiplicativeGroupAbelian>::Element>
{
    type Element: MultiplicativeGroupAbelianElement<Parent=Self>;
    fn is_additive_quasigroup(&self) -> bool { true }
}

pub trait MultiplicativeGroupAbelianElement:
    GroupAbelianElement<Multiplicative, Parent=<Self as MultiplicativeGroupAbelianElement>::Parent>
{
    type Parent: MultiplicativeGroupAbelian<Element=Self>;
}

impl<T> MultiplicativeGroupAbelian for T
where
    T: GroupAbelian<Multiplicative> 
{
    type Element = <T as GroupAbelian<Multiplicative>>::Element;
}

impl<T> MultiplicativeGroupAbelianElement for T
where
    T: GroupAbelianElement<Multiplicative>
{
    type Parent = <T as GroupAbelianElement<Multiplicative>>::Parent;
}
