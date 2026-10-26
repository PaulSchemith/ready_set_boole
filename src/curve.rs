/// Z-order (Morton) curve: interleaves the bits of x (even positions) and y (odd positions)
/// into a 32-bit index, then divides by 2^32. Dividing by a power of two is exact in f64,
/// so the mapping is a bijection onto {k / 2^32} and `reverse_map` recovers it exactly.
pub fn map(x: u16, y: u16) -> f64 {
    let mut z: u32 = 0;
    for i in 0..16 {
        z |= (((x >> i) & 1) as u32) << (2 * i);
        z |= (((y >> i) & 1) as u32) << (2 * i + 1);
    }
    z as f64 / 4_294_967_296.0
}

/// Inverse of `map`. Returns (0, 0) with an error message if `n` is not a valid image.
pub fn reverse_map(n: f64) -> (u16, u16) {
    let scaled = n * 4_294_967_296.0;
    if !(0.0..4_294_967_296.0).contains(&scaled) || scaled != (scaled as u32) as f64 {
        eprintln!("Error: {n} is not in the image of map");
        return (0, 0);
    }
    let z = scaled as u32;
    let (mut x, mut y) = (0u16, 0u16);
    for i in 0..16 {
        x |= (((z >> (2 * i)) & 1) as u16) << i;
        y |= (((z >> (2 * i + 1)) & 1) as u16) << i;
    }
    (x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip() {
        for &(x, y) in &[(0, 0), (1, 0), (0, 1), (65535, 65535), (12345, 54321), (65535, 0)] {
            assert_eq!(reverse_map(map(x, y)), (x, y));
        }
        assert_eq!(map(0, 0), 0.0);
        assert!(map(65535, 65535) < 1.0);
    }

    #[test]
    fn round_trip_dense() {
        for x in (0..=65535u32).step_by(257) {
            for y in (0..=65535u32).step_by(263) {
                let (x, y) = (x as u16, y as u16);
                assert_eq!(reverse_map(map(x, y)), (x, y));
            }
        }
    }
}
