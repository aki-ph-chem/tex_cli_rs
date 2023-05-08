use std::fs;
/// function to make directory structure
/// 存在するディレクトリをmkdirしようとしたときのエラー処理がいまいち
pub fn mkdir(path: &str) -> std::io::Result<()> {
    match fs::create_dir_all(path) {
        Ok(_) => Ok(()),
        Err(e) => {
            if e.kind() == std::io::ErrorKind::AlreadyExists {
                return Err(e);
            }
            Err(e)
        }
    }
}

#[cfg(test)]
mod tests_mkdirs {
    use super::*;
    use std::env;
    use std::path::PathBuf;

    #[test]
    /// mkdir test_dir 
    fn test_mkdir() -> std::io::Result<()> {
        let mut dir_path = PathBuf::new();
        dir_path.push(env::var("CARGO_MANIFEST_DIR").unwrap());
        dir_path.push("target");
        dir_path.push("test_dir");

        mkdir(dir_path.to_str().unwrap())?;
        assert_eq!(true, dir_path.exists());
        fs::remove_dir_all(dir_path)?;

        Ok(())
    }

    #[test]
    /// mkdir test_dir
    fn test_mkdir_same_dir() -> std::io::Result<()> {
        let mut dir_path = PathBuf::new();
        dir_path.push(env::var("CARGO_MANIFEST_DIR").unwrap());
        dir_path.push("target");
        dir_path.push("test_same_dir");

        // onece
        mkdir(dir_path.to_str().unwrap())?; 
        // twice
        if let Err(e) = mkdir(dir_path.to_str().unwrap()) {
            //eprintln!("Error: {}", e);
            panic!("Error: {}", e);
        } 
        fs::remove_dir_all(dir_path)?;

        Ok(())
    }

    #[test]
    /// mkdir -p foo/fuga
    fn test_mkdir_p() -> std::io::Result<()> {
        let mut dir_path = PathBuf::new();
        dir_path.push(env::var("CARGO_MANIFEST_DIR").unwrap());
        dir_path.push("target");
        dir_path.push("test_dir_parent/test_dir_child");

        mkdir(dir_path.to_str().unwrap())?;
        assert_eq!(true, dir_path.exists());
        fs::remove_dir_all(dir_path)?;

        Ok(())
    }
}
