pub trait ToFile {
    fn save(&self, path: &str) -> std::io::Result<()>;

    fn load(&mut self, path: &str) -> std::io::Result<()>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use tempfile::tempdir;

    // Implement ToFile for a simple test struct
    #[derive(Debug, PartialEq)]
    struct TestData {
        content: String,
    }

    impl ToFile for TestData {
        fn save(&self, path: &str) -> std::io::Result<()> {
            let mut file = std::fs::File::create(path)?;
            file.write_all(self.content.as_bytes())?;
            Ok(())
        }

        fn load(&mut self, path: &str) -> std::io::Result<()> {
            self.content = std::fs::read_to_string(path)?;
            Ok(())
        }
    }

    #[test]
    fn test_save_and_load() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_data.txt");
        let file_path_str = file_path.to_str().unwrap();

        let data = TestData {
            content: "Hello, World!".to_string(),
        };

        // Test save
        assert!(data.save(file_path_str).is_ok());

        // Verify file was created and contains correct data
        let saved_content = fs::read_to_string(file_path_str).unwrap();
        assert_eq!(saved_content, "Hello, World!");

        // Test load
        let mut loaded_data = TestData {
            content: String::new(),
        };
        assert!(loaded_data.load(file_path_str).is_ok());
        assert_eq!(loaded_data.content, "Hello, World!");
    }

    #[test]
    fn test_save_to_nonexistent_directory() {
        let data = TestData {
            content: "test".to_string(),
        };

        // Try to save to a path with nonexistent directory
        let result = data.save("/nonexistent/directory/file.txt");
        assert!(result.is_err());
    }

    #[test]
    fn test_load_from_nonexistent_file() {
        let mut data = TestData {
            content: String::new(),
        };

        // Try to load from nonexistent file
        let result = data.load("/nonexistent/file.txt");
        assert!(result.is_err());
    }

    #[test]
    fn test_save_empty_content() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("empty.txt");
        let file_path_str = file_path.to_str().unwrap();

        let data = TestData {
            content: String::new(),
        };

        assert!(data.save(file_path_str).is_ok());

        let saved_content = fs::read_to_string(file_path_str).unwrap();
        assert_eq!(saved_content, "");
    }

    #[test]
    fn test_load_empty_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("empty.txt");
        let file_path_str = file_path.to_str().unwrap();

        // Create empty file
        fs::File::create(file_path_str).unwrap();

        let mut data = TestData {
            content: "initial".to_string(),
        };

        assert!(data.load(file_path_str).is_ok());
        assert_eq!(data.content, "");
    }
}