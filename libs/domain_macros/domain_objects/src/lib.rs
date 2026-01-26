pub trait EntityTrait: Eq {
    type Identity<'a>
    where
        Self: 'a;

    fn identity(&self) -> Self::Identity<'_>;

    fn eq<'b>(&'b self, other: &'b Self) -> bool
    where
        Self::Identity<'b>: Eq,
    {
        self.identity() == other.identity()
    }
}
