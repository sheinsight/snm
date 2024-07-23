use std::{fs, ops::Not};

use snm_atom::atom::AtomTrait;
use snm_download_builder::{DownloadBuilder, WriteStrategy};
use snm_utils::snm_error::SnmError;

pub async fn ensure_binary_path<T>(
    atom: &T,
    version: &String,
    is_check: bool,
) -> Result<String, SnmError>
where
    T: AtomTrait,
{
    if atom
        .get_anchor_file_path_buf(version.as_str())?
        .exists()
        .not()
    {
        let download_url = atom.get_download_url(version);

        let downloaded_file_path_buf = atom.get_downloaded_file_path_buf(version)?;

        DownloadBuilder::new()
            .retries(3)
            .timeout(atom.get_snm_config().get_download_timeout_secs())
            .write_strategy(WriteStrategy::WriteAfterDelete)
            .download(&download_url, &downloaded_file_path_buf)
            .await?;

        let runtime_dir_path_buf = atom.get_runtime_dir_path_buf(version)?;

        if is_check {
            check(version, atom).await?;
        }

        atom.decompress_download_file(&downloaded_file_path_buf, &runtime_dir_path_buf)?;

        if let Some(parent) = downloaded_file_path_buf.parent() {
            fs::remove_dir_all(parent)?;
        }
    }

    let binary = atom.get_runtime_binary_dir_string(version.as_str())?;

    return Ok(binary);
}

async fn check<T>(version: &str, atom: &T) -> Result<(), SnmError>
where
    T: AtomTrait,
{
    let downloaded_file_path_buf = atom.get_downloaded_file_path_buf(version)?;

    let expect = atom.get_expect_shasum(version).await?;

    let actual = atom.get_actual_shasum(&downloaded_file_path_buf).await?;

    if actual.is_none() || expect.is_none() {
        fs::remove_file(&downloaded_file_path_buf)?;
        return Err(SnmError::ShasumError {
            file_path: downloaded_file_path_buf.display().to_string(),
            expect: "None".to_string(),
            actual: "None".to_string(),
        });
    }

    if actual.eq(&expect).not() {
        fs::remove_file(&downloaded_file_path_buf)?;
        return Err(SnmError::ShasumError {
            file_path: downloaded_file_path_buf.display().to_string(),
            expect: expect.unwrap_or("None".to_string()),
            actual: actual.unwrap_or("None".to_string()),
        });
    }

    Ok(())
}
