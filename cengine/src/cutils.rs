pub fn coords_to_index(x: f32, y: f32, width: usize) -> usize {
    // if x > width { (y * width) + x } else { 0 }
    (y as usize * width) + x as usize
}
