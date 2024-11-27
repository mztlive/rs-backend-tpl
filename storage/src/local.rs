use std::path::{Path, PathBuf};
use tokio::{fs, io::AsyncWriteExt};

use crate::{Error, Result};

/// 本地文件存储实现
pub struct LocalStorage {
    /// 基础存储路径
    base_path: PathBuf,
}

impl LocalStorage {
    /// 创建新的本地存储实例
    ///
    /// # 参数
    ///
    /// * `base_path` - 基础存储路径
    ///
    /// # 错误
    ///
    /// 如果基础路径不存在或无法创建，将返回错误
    pub async fn new<P: AsRef<Path>>(base_path: P) -> Result<Self> {
        let base_path = base_path.as_ref().to_path_buf();

        // 确保基础路径存在
        if !base_path.exists() {
            fs::create_dir_all(&base_path).await?;
        }

        Ok(Self { base_path })
    }

    /// 保存文件
    ///
    /// # 参数
    ///
    /// * `path` - 相对存储路径
    /// * `content` - 文件内容
    ///
    /// # 错误
    ///
    /// 如果文件无法保存，将返回错误
    pub async fn save<P: AsRef<Path>>(&self, path: P, content: &[u8]) -> Result<()> {
        let full_path = self.get_full_path(path)?;

        // 确保父目录存在
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent).await?;
        }

        // 写入文件
        let mut file = fs::File::create(&full_path).await?;
        file.write_all(content).await?;
        file.sync_all().await?;

        Ok(())
    }

    /// 读取文件
    ///
    /// # 参数
    ///
    /// * `path` - 相对存储路径
    ///
    /// # 返回
    ///
    /// 返回文件内容的字节数组
    ///
    /// # 错误
    ///
    /// 如果文件不存在或无法读取，将返回错误
    pub async fn read<P: AsRef<Path>>(&self, path: P) -> Result<Vec<u8>> {
        let full_path = self.get_full_path(path)?;

        if !full_path.exists() {
            return Err(Error::NotFound);
        }

        Ok(fs::read(&full_path).await?)
    }

    /// 删除文件
    ///
    /// # 参数
    ///
    /// * `path` - 相对存储路径
    ///
    /// # 错误
    ///
    /// 如果文件不存在或无法删除，将返回错误
    pub async fn delete<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let full_path = self.get_full_path(path)?;

        if !full_path.exists() {
            return Err(Error::NotFound);
        }

        fs::remove_file(&full_path).await?;
        Ok(())
    }

    /// 检查��件是否存在
    ///
    /// # 参数
    ///
    /// * `path` - 相对存储路径
    pub async fn exists<P: AsRef<Path>>(&self, path: P) -> bool {
        match self.get_full_path(path) {
            Ok(full_path) => full_path.exists(),
            Err(_) => false,
        }
    }

    /// 获取完整文件路径
    ///
    /// # 参数
    ///
    /// * `path` - 相对存储路径
    ///
    /// # 错误
    ///
    /// 如果路径无效，将返回错误
    fn get_full_path<P: AsRef<Path>>(&self, path: P) -> Result<PathBuf> {
        let path = path.as_ref();

        // 检查路径是否包含 ..
        if path.components().any(|c| c.as_os_str() == "..") {
            return Err(Error::PathError("路径不能包含 ..".to_string()));
        }

        Ok(self.base_path.join(path))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_save_and_read() -> Result<()> {
        let temp_dir = tempdir()?;
        let storage = LocalStorage::new(temp_dir.path()).await?;

        let content = b"Hello, World!";
        storage.save("test.txt", content).await?;

        let read_content = storage.read("test.txt").await?;
        assert_eq!(read_content, content);

        Ok(())
    }

    #[tokio::test]
    async fn test_delete() -> Result<()> {
        let temp_dir = tempdir()?;
        let storage = LocalStorage::new(temp_dir.path()).await?;

        storage.save("test.txt", b"test").await?;
        assert!(storage.exists("test.txt").await);

        storage.delete("test.txt").await?;
        assert!(!storage.exists("test.txt").await);

        Ok(())
    }

    #[tokio::test]
    async fn test_not_found() {
        let temp_dir = tempdir().unwrap();
        let storage = LocalStorage::new(temp_dir.path()).await.unwrap();

        assert!(matches!(
            storage.read("nonexistent.txt").await,
            Err(Error::NotFound)
        ));
    }

    #[tokio::test]
    async fn test_invalid_path() {
        let temp_dir = tempdir().unwrap();
        let storage = LocalStorage::new(temp_dir.path()).await.unwrap();

        assert!(matches!(
            storage.save("../test.txt", b"test").await,
            Err(Error::PathError(_))
        ));
    }
}
