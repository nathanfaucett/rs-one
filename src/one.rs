

pub trait One {
    fn one() -> Self;
    fn is_one(&self) -> bool;
}

macro_rules! trait_one {
    ($t:ident) => (
        impl One for $t {
            #[inline(always)]
            fn one() -> Self { 1 }
            #[inline(always)]
            fn is_one(&self) -> bool { *self == 1 }
        }
    );
}

macro_rules! trait_one_float {
    ($t:ident) => (
        impl One for $t {
            #[inline(always)]
            fn one() -> Self { 1.0 }
            #[inline(always)]
            fn is_one(&self) -> bool { *self == 1.0 }
        }
    );
}

trait_one!(usize);
trait_one!(u8);
trait_one!(u16);
trait_one!(u32);
trait_one!(u64);

trait_one!(isize);
trait_one!(i8);
trait_one!(i16);
trait_one!(i32);
trait_one!(i64);

trait_one_float!(f32);
trait_one_float!(f64);


impl One for bool {
    #[inline(always)]
    fn one() -> bool { true }
    #[inline(always)]
    fn is_one(&self) -> bool { *self }
}
