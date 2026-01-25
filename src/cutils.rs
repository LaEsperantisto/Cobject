pub fn coords_to_index(x: u32, y: u32, width: usize) -> usize {
    (y as usize * width) + x as usize
}
