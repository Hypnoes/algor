pub mod binary_search;
pub mod binomail;
pub mod matrix;

struct Sort<I, F> {
    iter: I,
    cmp: F
}

impl<I, F> Sort<I, F> {
    pub fn new(iter: I, cmp: F) -> Sort<I, F> {
        Sort{iter, cmp}
    }
}

impl<B, I, F> Iterator for Sort<I, F>
    where
        I: Iterator,
        F: Fn(I::Item, I::Item) -> bool
{
    type Item = B;
    
    fn next(&mut self) -> Option<B> {
        self.iter.next().sort(self.cmp)
    }

}

trait ISort<I, F> {
    fn sort(&self, cmp: F) -> Sort<I, F>;
}

impl<I, F> ISort<I, F> for I
    where
        I: Iterator,
        F: Fn(I::Item, I::Item) -> bool
{
    fn sort(&self, cmp: F) -> Sort<I, F> {
        Sort {
            iter: *self,
            cmp
        }
    }
}