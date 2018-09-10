pub mod data;

#[cfg(test)]
pub mod test{
    use data::Parent;
    #[test]
    fn test_parents() {
        let parent = Parent::new(Some(32),String::from("William"), Some(String::from("Logan")), String::from("Porter"));
    }
}
