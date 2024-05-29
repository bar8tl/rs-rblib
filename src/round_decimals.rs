// round_decimals: Performs rounding of floating point numbers to specific decimal
// positions
pub fn round_decimals(x: f32, y: u32) -> f32 {
  let y = 10i32.pow(y) as f32;
  (x * y).round() / y
}
