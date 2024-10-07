// TODO: Implement the `From` trait for the `WrappingU32` type to make `example` compile.

pub struct WrappingU32 {
    value: u32,
}


// impl<T, U> Into<U> for T
// where
//     U: From<T>,
// {
//     fn into(self) -> U {
//         U::from(self)
//     }
// }

// the order of T and U in impl<T, U> is important because it determines how the Into<U> trait is implemented for the type T.

// In this context, T is the type you're converting from, and U is the type you're converting to. When you write impl<T, U> Into<U> for T, you're stating that for any type T, if there's a way to convert T into U (provided that U implements the From<T> trait), then the Into<U> trait is implemented for T.

// If you were to reverse them, like impl<U, T> Into<U> for T, it wouldn't make sense for the conversion because T wouldn't be known to be the type you are converting from in the context of Into<U>. So, the order is significant in defining the relationship between the types in the implementation.

impl From<u32> for WrappingU32{
    // For explicit clarity
    // fn from(u32_value: u32) -> Self {
    //     WrappingU32 { value: u32_value }
    // }

    // another way
    fn from(value: u32) -> Self {
        WrappingU32 { value }
    }
}

fn example() {
    let wrapping: WrappingU32 = 42.into();
    let wrapping = WrappingU32::from(42);
}
