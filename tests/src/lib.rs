#[cfg(test)]
mod tests {
    use default2::Default2;

    #[derive(Default2)]
    struct TestDefault {
        #[default(10)]
        integer: i32,
        #[default("test".into())]
        string: String,
        #[default(gen_usize())]
        custom: usize,
        empty: u64,
    }

    #[test]
    fn test_default_setter() {
        let t = TestDefault::default();
        assert_eq!(t.integer, 10);
        assert_eq!(t.string, "test");
        assert_eq!(t.custom, 1);
        assert_eq!(t.empty, 0);
    }

    fn gen_usize() -> usize {
        1
    }
}
