// TODO: Implement the `From` trait for the `WrappingU32` type to make `example` compile.
//
pub struct WrappingU32 {
    value: u32,
}

impl From<i32> for WrappingU32 {
    fn from(value: i32) -> WrappingU32 {
        let new_val = match u32::try_from(value) {
            Ok(u32) => u32,
            Err(e) => 0,
        };
        WrappingU32 { value: new_val }
    }
}

fn example() {
    let wrapping: WrappingU32 = 42.into();
    let wrapping = WrappingU32::from(42);
}
