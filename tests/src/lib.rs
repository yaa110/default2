use default2::default;

default! {
    struct TestDefault {
        id: i32 = 10,
        name: String = "main".into(),
        cpus: usize = gen_usize(),
        vector: Vec<u64> = vec![1, 2, 3],
        payload: u64,
    }
}

default! {
    struct TestDefaultWithWeirdSpacing {
        id: i32=10,
        name: String ="main".into(),
        cpus: usize= gen_usize(),
        vector: Vec<u64> =vec![1, 2, 3],
        payload: u64,
    }
}

default! {
    #[derive(Debug, PartialEq)]
    struct TestWithOtherDerives {
        id: i32 = 100,
        name: String,
    }
}

default! {
    /// This is a test struct with doc comments.
    struct TestWithDocs {
        /// This is the ID.
        id: i32 = 100,

        #[doc = "This is the name."]
        name: String,
    }
}

fn gen_usize() -> usize {
    1
}

#[test]
fn test_default_setter() {
    let t = TestDefault::default();
    assert_eq!(t.id, 10);
    assert_eq!(t.name, "main");
    assert_eq!(t.cpus, 1);
    assert_eq!(t.vector, &[1, 2, 3]);
    assert_eq!(t.payload, 0);
}

#[test]
fn test_default_with_weird_spacing() {
    let t = TestDefaultWithWeirdSpacing::default();
    assert_eq!(t.id, 10);
    assert_eq!(t.name, "main");
    assert_eq!(t.cpus, 1);
    assert_eq!(t.vector, &[1, 2, 3]);
    assert_eq!(t.payload, 0);
}

#[test]
fn test_init_with_default_rest() {
    let t = TestDefault {
        id: 20,
        payload: 100,
        ..Default::default()
    };
    assert_eq!(t.id, 20);
    assert_eq!(t.name, "main");
    assert_eq!(t.cpus, 1);
    assert_eq!(t.vector, &[1, 2, 3]);
    assert_eq!(t.payload, 100);
}

#[test]
fn test_struct_with_other_derives() {
    let s1 = TestWithOtherDerives::default();
    let s2 = TestWithOtherDerives {
        id: 100,
        name: String::new(),
    };

    assert_eq!(s1, s2); // This requires PartialEq
    println!("{:?}", s1); // This requires Debug
}

#[test]
fn test_struct_with_docs() {
    let s = TestWithDocs::default();
    assert_eq!(s.id, 100);
    assert_eq!(s.name, "");
}
