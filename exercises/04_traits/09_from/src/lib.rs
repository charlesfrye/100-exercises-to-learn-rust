pub struct WrappingU32 {
    value: u32,
}

impl From<WrappingU32> for u32 {
    fn from(w: WrappingU32) -> u32 {
        w.value
    }
}

impl From<u32> for WrappingU32 {
    fn from(u: u32) -> WrappingU32 {
        WrappingU32 { value: u }
    }
}

// interesting asymmetry -- no reverse blanket impl

// impl Into<WrappingU32> for u32 {
//     fn into(self) -> WrappingU32 {
//         WrappingU32 { value: self }
//     }
// }

// impl Into<u32> for WrappingU32 {
//     fn into(self) -> u32 {
//         self.value
//     }
// }

fn example() {
    let wrapping: WrappingU32 = 42.into();
    let wrapping = WrappingU32::from(42);
}
