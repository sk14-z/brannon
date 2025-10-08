pub(crate) trait Mask {
    type Output;

    fn mask(&self) -> usize;

    fn unmask(mask: usize) -> Self::Output;
}
