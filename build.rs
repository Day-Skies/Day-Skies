//! Generate typed resource + localization constants from `resource/` (day-build): the `res` module
//! in `src/lib.rs` surfaces `res::str::<key>()` for every Fluent message, checked at compile time.
fn main() {
    day_build::generate_resources().expect("day-build: resource codegen");
}
