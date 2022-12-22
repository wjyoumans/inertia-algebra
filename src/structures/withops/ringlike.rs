use crate::ops::*;
use crate::structures as structures;


pub trait RingOps: AddOps
    + NegOps
    + SubOps
    + MulOps
{}

pub trait NCRing:
    structures::NCRing<Element=<Self as NCRing>::Element>
{
    type Element: NCRingElement<Parent=Self>;
    fn is_ncring(&self) -> bool { true }
}

pub trait NCRingElement:
    structures::NCRingElement<Parent=<Self as NCRingElement>::Parent>
    + RingOps
{
    type Parent: NCRing<Element=Self>;
}

impl<T> NCRing for T
where
    T: structures::NCRing,
    <T as structures::NCRing>::Element: RingOps
{
    type Element = <T as structures::NCRing>::Element;
}

impl<T> NCRingElement for T
where
    T: structures::NCRingElement + RingOps
{
    type Parent = <T as structures::NCRingElement>::Parent;
}

pub trait Ring:
    structures::Ring<Element=<Self as Ring>::Element>
{
    type Element: RingElement<Parent=Self>;
    fn is_ring(&self) -> bool { true }
}

pub trait RingElement:
    structures::RingElement<Parent=<Self as RingElement>::Parent>
    + RingOps
{
    type Parent: Ring<Element=Self>;
}


impl<T> Ring for T
where
    T: structures::Ring,
    <T as structures::Ring>::Element: RingOps 
{
    type Element = <T as structures::Ring>::Element;
}

impl<T> RingElement for T
where
    T: structures::RingElement 
    + RingOps
{
    type Parent = <T as structures::RingElement>::Parent;
}


pub trait FieldOps: RingOps + InvOps + DivOps {}

pub trait Field:
    structures::Field<Element=<Self as Field>::Element>
{
    type Element: FieldElement<Parent=Self>;
    fn is_field(&self) -> bool { true }
}

pub trait FieldElement:
    structures::FieldElement<Parent=<Self as FieldElement>::Parent>
    + FieldOps
{
    type Parent: Field<Element=Self>;
}


impl<T> Field for T
where
    T: structures::Field,
    <T as structures::Field>::Element: FieldOps 
{
    type Element = <T as structures::Field>::Element;
}

impl<T> FieldElement for T
where
    T: structures::FieldElement 
    + FieldOps
{
    type Parent = <T as structures::FieldElement>::Parent;
}