pub trait TryDefault {
    fn try_default() -> anyhow::Result<Self>
    where
        Self: Sized;
}
