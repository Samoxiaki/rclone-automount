use crate::models::{Config, Environment, MountPoint};
use tokio::fs;
use tokio::process::Command;
use tokio::task::JoinSet;
use std::collections::HashMap;
use std::path::Path;
use std::process::Stdio;

pub async fn read_config(path: String) -> Result<Config, String> {
	tracing::info!("Reading configuration at {}", path);
	
	match fs::read_to_string(&path).await {
		Ok(content) => {
			match toml::from_str(&content) {
				Ok(config) => Ok(config),
				Err(e) => Err(format!("Error parsing TOML: {}", e)),
			}
		},
		Err(e) => return Err(format!("Error reading file {}: {}", path, e)),
	}

   
}


pub async fn check_mountpoint_dir(mountpoint: &MountPoint) -> Result<(), String> {
    let path = Path::new(&mountpoint.mount_dir);

    if path.exists() {
        if path.is_dir() {
            Ok(())
        } else {
            Err(format!("The mountpoint {} exists but is not a directory", mountpoint.mount_dir))
        }
    } else {
       Err(format!("The mountpoint {} does not exist", mountpoint.mount_dir))
    }
}


pub async fn rclone_mount(environment: &Environment, mountpoint: &MountPoint, mountpoint_name: &str) -> Result<(), String> {
    check_mountpoint_dir(mountpoint).await?;

    let _ = mountpoint.unmount_command(environment)
		.stderr(Stdio::null())
        .status()
        .await;

	tracing::info!("Mounting {} at {}", mountpoint_name, mountpoint.mount_dir);

    let status = mountpoint.mount_command(environment)
        .status()
        .await
        .map_err(|e| format!("Failed to start mount command: {}", e))?;


    if !(status.success() || (status.code().unwrap_or(0) >= 128 && status.code().unwrap_or(0) <= 143)) {
        return Err(format!("Mount command for {} exited with status: {}", mountpoint_name, status));
    } 
	return Ok(());

	
}

pub async fn rclone_unmount(environment: &Environment, mountpoints: &HashMap<String, MountPoint>) {

    let unmount_data: Vec<(String, Command, String)> = mountpoints.iter()
        .map(|(key, mount_point)| {
            let cmd = mount_point.unmount_command(environment);

            let mount_dir = mount_point.mount_dir.clone();
            (key.clone(), cmd, mount_dir)
        })
        .collect();

    let mut join_set = JoinSet::new();

    for (key, mut cmd, mount_dir) in unmount_data {
        join_set.spawn(async move {
            match cmd.status().await
{
                Ok(status) if status.success() => {
                    tracing::info!("Unmounted {} at {} successfully", key, mount_dir);
                }
                Ok(status) => {
                    tracing::error!("Failed to unmount {} at {} with status: {:?}", key, mount_dir, status);
                }
                Err(e) => {
                    tracing::error!("Error running unmount for {} at {}: {}", key, mount_dir, e);
                }
            }
        });
    }

    while let Some(res) = join_set.join_next().await {
        if let Err(e) = res {
            tracing::error!("Task failed: {:?}", e);
        }
    }

    tracing::info!("All mountpoints unmounted. Exiting.");
}

