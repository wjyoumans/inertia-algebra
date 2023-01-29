
use crate::*;

pub trait MatrixSpace<T: Ring>:
    AdditiveGroupAbelian<Element=<Self as MatrixSpace<T>>::Element>
{
    type Element: MatrixSpaceElement<T, Parent=Self>;

    fn init<D: Into<u64>>(ring: &T, nrows: D, ncols: D) -> Self;

    /// Return a reference to the base ring.
    fn base_ring(&self) -> &T;

    fn nrows(&self) -> usize;

    fn ncols(&self) -> usize;

    #[inline]
    fn is_generic(&self) -> bool {
        false
    }
}

pub trait MatrixSpaceElement<T: Ring>:
    AdditiveGroupAbelianElement<Parent=<Self as MatrixSpaceElement<T>>::Parent>
{
    type Parent: MatrixSpace<T, Element=Self>;

    //type BorrowCoeff<'a>: Deref<Target=T>;
    //type BorrowCoeffMut<'a>: DerefMut<Target=T>;
    
    /// Return a reference to the base ring.
    fn base_ring(&self) -> &T;

    fn len(&self) -> usize;
    
    fn nrows(&self) -> usize;

    fn ncols(&self) -> usize;

    fn get_entry(&self, i: usize, j: usize) -> Option<Elem<T>>;

    fn set_entry(&mut self, i: usize, j: usize, entry: Elem<T>) -> Option<Elem<T>>;
    
    fn get_entries(&self) -> Vec<Elem<T>>;
    
    #[inline]
    fn is_generic(&self) -> bool {
        false
    }
}
