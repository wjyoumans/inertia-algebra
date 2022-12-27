use crate::ops::*;
use crate::structures::*;


pub trait RingOps: AddOps
    + NegOps
    + SubOps
    + MulOps
{}

pub trait NCRing:
    AbstractNCRing<Element=<Self as NCRing>::Element>
{
    type Element: NCRingElement<Parent=Self>;
    fn is_ncring(&self) -> bool { true }
}

pub trait NCRingElement:
    AbstractNCRingElement<Parent=<Self as NCRingElement>::Parent>
    + RingOps
{
    type Parent: NCRing<Element=Self>;
}

impl<T> NCRing for T
where
    T: AbstractNCRing,
    <T as AbstractNCRing>::Element: RingOps
{
    type Element = <T as AbstractNCRing>::Element;
}

impl<T> NCRingElement for T
where
    T: AbstractNCRingElement + RingOps
{
    type Parent = <T as AbstractNCRingElement>::Parent;
}

pub trait Ring:
    AbstractRing<Element=<Self as Ring>::Element>
{
    type Element: RingElement<Parent=Self>;
    fn is_ring(&self) -> bool { true }
}

pub trait RingElement:
    AbstractRingElement<Parent=<Self as RingElement>::Parent>
    + RingOps
{
    type Parent: Ring<Element=Self>;
}

impl<T> Ring for T
where
    T: AbstractRing,
    <T as AbstractRing>::Element: RingOps 
{
    type Element = <T as AbstractRing>::Element;
}

impl<T> RingElement for T
where
    T: AbstractRingElement 
    + RingOps
{
    type Parent = <T as AbstractRingElement>::Parent;
}


pub trait FieldOps: RingOps + InvOps + DivOps {}

pub trait Field:
    AbstractField<Element=<Self as Field>::Element>
{
    type Element: FieldElement<Parent=Self>;
    fn is_field(&self) -> bool { true }
}

pub trait FieldElement:
    AbstractFieldElement<Parent=<Self as FieldElement>::Parent>
    + FieldOps
{
    type Parent: Field<Element=Self>;
}

impl<T> Field for T
where
    T: AbstractField,
    <T as AbstractField>::Element: FieldOps 
{
    type Element = <T as AbstractField>::Element;
}

impl<T> FieldElement for T
where
    T: AbstractFieldElement 
    + FieldOps
{
    type Parent = <T as AbstractFieldElement>::Parent;
}
