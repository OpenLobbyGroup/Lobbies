use serde::{de::DeserializeOwned, Serialize};
use std::path::Path;
use tokio::{fs, io};

pub async fn upload_to_path<T>(path: &str, data: T) -> io::Result<()>
where
    T: Serialize + Send,
{
    // Create parent directories if they do not exist
    if let Some(parent) = Path::new(path).parent()
    {
        fs::create_dir_all(parent).await?;
    }

    // Serialize the data
    let buf = bincode::serialize(&data).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    // Write serialized data to the file asynchronously
    fs::write(path, buf).await?;

    Ok(())
}

/// Generalized async function to load any deserializable data from a specified path
pub async fn load_from_path<T>(path: &str) -> io::Result<T>
where
    T: DeserializeOwned + Default,
{
    if Path::new(path).exists()
    {
        // Read the file contents asynchronously
        let buf = fs::read(path).await?;

        // Deserialize the buffer into the specified type `T`
        let data = bincode::deserialize(&buf).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        Ok(data)
    }
    else
    {
        // Return the default instance if the file doesn't exist
        Ok(T::default())
    }
}
