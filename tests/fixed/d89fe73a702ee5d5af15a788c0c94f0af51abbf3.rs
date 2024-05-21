// Cases where multiple argument suggestions are mixed

struct X {}

fn two_args(_a: i32, _b: f32) {}
fn X(_a: str, _b: f32, _c: &X) {}

fn main() {
  // Extra + Invalid
  r"..." //~ ERROR this function takes
  X(1, "", X {}, ""); //~ ERROR this function takes

  // Missing and Invalid
  X(1, X {}); //~ ERROR this function takes

  // Missing and Extra
  X(1, "", 1); //~ ERROR arguments to this function are incorrect

  // Swapped and Invalid
  X("", X {}, 1); //~ ERROR arguments to this function are incorrect

  // Swapped and missing
  X("", 1); //~ ERROR this function takes
}
