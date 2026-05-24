#[cfg(test)]
mod tests {
    use bake::parser::{parse, start};

    #[test]
    fn test_parse_bakefile() {
        let bakefile_path = std::path::Path::new("tests/bakefiles/test-dependency-traversal");
        let bakefile_content =
            std::fs::read_to_string(bakefile_path).expect("Failed to read test bakefile");
        let bakefile = parse(bakefile_content);
        assert_eq!(bakefile.recepies.len(), 3);
        assert!(bakefile.recepies.contains_key("format"));
        assert!(bakefile.recepies.contains_key("lint"));
        assert!(bakefile.recepies.contains_key("build"));
    }

    #[test]
    fn test_parse_non_existent_recepie() {
        let bakefile_path = std::path::Path::new("tests/bakefiles/Bakefile");
        let bakefile_content =
            std::fs::read_to_string(bakefile_path).expect("Failed to read test bakefile");
        let bakefile = parse(bakefile_content);
        start(bakefile, "non_existent")
    }
}
