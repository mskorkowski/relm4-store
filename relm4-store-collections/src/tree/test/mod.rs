

mod tree {
    // use super::super::Tree;
    /// Test creation with default configuration
    #[test]
    fn default() {
        // let tree = Tree::<usize, usize>::default();

        // assert_eq!(tree.len(), 0, "Length of freshly created tree should be `0`");
        // assert_eq!(tree.is_empty(), true, "Freshly created tree should be empty");
    }
}

mod tree_configuration {
    use super::super::TreeConfiguration;

    #[test]
    fn default() {
        let configuration = TreeConfiguration::default();

        assert_eq!(configuration.capacity(), 11, "Maximum number of elements in node for default configuration is 11");
    }

    #[test]
    fn new() {
        let configuration = TreeConfiguration::new(20);

        assert_eq!(configuration.capacity(), 19, "Maximum number of elements in node for configuration")
    }
}