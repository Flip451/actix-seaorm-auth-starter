pub(crate) trait DbErrorMapper {
    fn is_unique_violation(&self) -> bool;
    fn constraint_name(&self) -> Option<&str>;
}
