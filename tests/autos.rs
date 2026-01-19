use areia::{auto_creator, auto_deletor};

#[test]
fn auto_creating_basic() {
    let path = "auto_test_dir/test_file.txt";

    assert!(auto_creator(path).is_ok());
    assert!(std::path::Path::new(path).exists());

    assert!(auto_deletor("auto_test_dir").is_ok());
}

#[test]
fn auto_creating_nested() {
    let path = "a_test_dir/auto_test_dir/test_file.txt";

    assert!(auto_creator(path).is_ok());
    assert!(std::path::Path::new(path).exists());

    assert!(auto_deletor("a_test_dir").is_ok());
}

#[test]
fn auto_deleting_dir_structure1() {
    create_dir_structure(1);

    assert!(auto_deletor("parent1/child/").is_ok());
    assert!(!std::path::Path::new("parent1/child/a.file").exists());
    assert!(!std::path::Path::new("parent1/child/").exists());
    assert!(std::path::Path::new("parent1/another.file").exists());

    assert!(auto_deletor("parent1/").is_ok());
    assert!(!std::path::Path::new("parent1/").exists());
}

#[test]
fn auto_deleting_dir_structure2() {
    create_dir_structure(2);

    assert!(auto_deletor("parent2/my.file").is_ok());
    assert!(!std::path::Path::new("parent2/my.file").exists());
    assert!(!std::path::Path::new("parent2/").exists());
}

#[test]
fn auto_deleting_dir_structure3() {
    create_dir_structure(3);

    assert!(auto_deletor("parent3/child/a.file").is_ok());
    assert!(!std::path::Path::new("parent3/child/a.file").exists());
    assert!(!std::path::Path::new("parent3/child/").exists());
    assert!(std::path::Path::new("parent3/another.file").exists());

    assert!(auto_deletor("parent3/").is_ok());
    assert!(!std::path::Path::new("parent3/").exists());
}

#[test]
fn auto_deleting_dir_structure4() {
    create_dir_structure(4);

    assert!(auto_deletor("parent4/child2/b.file").is_ok());
    assert!(!std::path::Path::new("parent4/child2/b.file").exists());
    assert!(!std::path::Path::new("parent4/child2/c.file").exists());
    assert!(!std::path::Path::new("parent4/child2/").exists());
    assert!(std::path::Path::new("parent4/another.file").exists());

    assert!(auto_deletor("parent4/").is_ok());
}

/// Creates the following directory structure:
/// parent{$num}/
/// ├── child/
/// │   └── a.file
/// ├── child2/
/// │   ├── b.file
/// │   └── c.file
/// ├── another.file
/// └── my.file
fn create_dir_structure(num: u8) {
    std::fs::create_dir_all(format!("parent{}/child", num)).unwrap();
    std::fs::create_dir_all(format!("parent{}/child2", num)).unwrap();
    std::fs::File::create(format!("parent{}/another.file", num)).unwrap();
    std::fs::File::create(format!("parent{}/my.file", num)).unwrap();
    std::fs::File::create(format!("parent{}/child/a.file", num)).unwrap();
    std::fs::File::create(format!("parent{}/child2/b.file", num)).unwrap();
    std::fs::File::create(format!("parent{}/child2/c.file", num)).unwrap();
}
