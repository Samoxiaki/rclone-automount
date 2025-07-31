use std::{collections::{HashMap, HashSet}, sync::Arc};

use rclone_automount::{models::{Environment, MountPoint}, tasks::{rclone_mount, rclone_unmount, read_config}, utils::expand_home};
use tokio::{sync::Mutex, task::JoinSet};
use tracing_subscriber;

#[tokio::main]
async fn main() {
	tracing_subscriber::fmt::init();

	let config_path = match std::env::args().nth(1) {
		Some(path) => expand_home(&path),
		None => {
			eprintln!("Usage: rclone-automount <path_to_toml_file>");
			std::process::exit(1);
		}
	};
	


	let config = match read_config(config_path).await {
		Ok(config) => config,
		Err(e) => {
			eprintln!("Error reading configuration: {}", e);
			std::process::exit(1);
		}
	};

	let environment = match config.environment {
		Some(env) => env,
		None => {
			tracing::info!("No environment configuration found in the TOML file, using defaults.");
			Environment::default()
		}
	};

	let mut mountpoints: HashMap<String, MountPoint> = HashMap::new();
	for (mountpoint_name, mountpoint) in config.mountpoints.into_iter() {
		if mountpoint.enabled.unwrap_or(true) {
			mountpoints.insert(mountpoint_name, mountpoint);
		} else {
			tracing::info!("Mountpoint {} is disabled, skipping.", mountpoint_name);
		}
	};


	let failed_mountpoints: Arc<Mutex<HashSet<String>>> = Arc::new(Mutex::new(HashSet::new()));
	let max_failed_mountpoints = mountpoints.len();
	let mut mountpoints_join_set: JoinSet<()> = JoinSet::new();
	
	for (name, mount_point) in mountpoints.iter() {
		
		let failed_mountpoints_c = Arc::clone(&failed_mountpoints);
		let environment_c = environment.clone();
		let name_c = name.clone();
		let mount_point_c = mount_point.clone();
		mountpoints_join_set.spawn( 
			async move {
				let result = rclone_mount(
					&environment_c,
					 &mount_point_c, 
					 &name_c
				).await;

				match result {
					Ok(()) => {
						tracing::info!("Process for {} finished successfully.", name_c);
					}
					Err(e) => {
						tracing::error!("Failed to mount {}: {}", name_c, e);
						let mut failed_mp_mgt = failed_mountpoints_c.lock().await;
						failed_mp_mgt.insert(name_c.clone());
						if failed_mp_mgt.len() >= max_failed_mountpoints {
							tracing::error!("All mountpoints failed to mount, exiting.");
							std::process::exit(1);
						}
					},
				}
			}
		);
	}


	let _ = tokio::signal::ctrl_c().await;
   	tracing::info!("Received SIGINT, unmounting all mount points...");


	let mut unmount_mountpoints: HashMap<String, MountPoint> = HashMap::new();
	for (name, mount_point) in mountpoints.iter() {
		if failed_mountpoints.lock().await.contains(name) {
			tracing::info!("Skipping unmount for {} as it failed to mount.", name);
			continue;
		} else {
			unmount_mountpoints.insert(name.clone(), mount_point.clone());
		}
	}
	

	rclone_unmount(&environment, &unmount_mountpoints).await;

	while let Some(res) = mountpoints_join_set.join_next().await {
		if let Err(e) = res {
			tracing::error!("Task failed: {:?}", e);
		}
	}

		
	
		
}
