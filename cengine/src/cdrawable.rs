pub trait CDrawable {
    fn draw(&self, pixels: &mut Vec<u32>, width: usize, height: usize);
}
