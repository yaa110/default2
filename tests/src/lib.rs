#[cfg(test)]
mod tests {
    #[derive(default2::Default)]
    struct TestDefault {
        #[default(10)]
        integer: i32,
        #[default("test".into())]
        string: String,
        #[default(gen_usize())]
        custom: usize,
        #[default(vec![1, 2, 3])]
        vector: Vec<u64>,
        empty: u64,
    }

    #[test]
    fn test_default_setter() {
        let t = TestDefault::default();
        assert_eq!(t.integer, 10);
        assert_eq!(t.string, "test");
        assert_eq!(t.custom, 1);
        assert_eq!(t.vector, &[1, 2, 3]);
        assert_eq!(t.empty, 0);
    }

    fn gen_usize() -> usize {
        1
    }
}
