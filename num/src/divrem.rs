pub trait DivRem {
    fn div_rem(self, other: Self) -> (Self, Self)
    where
        Self: Sized;
}