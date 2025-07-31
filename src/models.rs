use serde::{Deserialize, Serialize};
use serde_plain::to_string;
use tokio::process::Command;
use std::collections::HashMap;

use crate::utils::expand_home;

#[derive(Debug, Deserialize, Clone)]
pub struct Environment {
    pub rclone_bin: Option<String>,
	pub rclone_conf: Option<String>,
	pub fusermount_bin: Option<String>,
}

impl Environment {

	pub fn default() -> Self {
		Environment {
			rclone_bin: None,
			rclone_conf: None,
			fusermount_bin: None,
		}
	}
    pub fn rclone_bin(&self) -> &str {
        self.rclone_bin.as_deref().unwrap_or("/usr/bin/rclone")
    }

	pub fn rclone_conf(&self) -> &str {
		self.rclone_conf.as_deref().unwrap_or("~/.config/rclone/rclone.conf")
	}

	pub fn fusermount_bin(&self) -> &str {
		self.fusermount_bin.as_deref().unwrap_or("/usr/bin/fusermount")
	}
}


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum VfsCacheMode {
    Off,
    Minimal,
    Writes,
    Full,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    DEBUG,
    INFO,
    NOTICE,
    ERROR,
    CRITICAL,
}


#[derive(Debug, Deserialize, Clone)]
pub struct MountPoint {
    pub enabled: Option<bool>,
    pub remote: String,
    pub mount_dir: String,

    pub allow_other: Option<bool>,
    pub allow_non_empty: Option<bool>,
    pub allow_root: Option<bool>,
    pub default_permissions: Option<bool>,
    pub uid: Option<u32>,
    pub gid: Option<u32>,
    pub umask: Option<u32>,
    pub dir_perms: Option<String>,
    pub file_perms: Option<String>,
    pub link_perms: Option<String>,
    pub devname: Option<String>,
    pub mount_case_insensitive: Option<bool>,

	pub cache_dir: Option<String>,
    pub vfs_cache_mode: Option<VfsCacheMode>,
    pub vfs_cache_max_age: Option<String>,
    pub vfs_cache_max_size: Option<String>,
    pub vfs_cache_min_free_space: Option<String>,
    pub vfs_cache_poll_interval: Option<String>,
    pub buffer_size: Option<String>,
    pub vfs_block_norm_dupes: Option<bool>,
    pub vfs_case_insensitive: Option<bool>,
    pub vfs_disk_space_total_size: Option<String>,
    pub vfs_fast_fingerprint: Option<bool>,
    pub vfs_links: Option<bool>,
    pub vfs_metadata_extension: Option<String>,
    pub vfs_read_ahead: Option<String>,
    pub vfs_read_chunk_size: Option<String>,
    pub vfs_read_chunk_size_limit: Option<String>,
    pub vfs_read_chunk_streams: Option<u32>,
    pub vfs_read_wait: Option<String>,
    pub vfs_refresh: Option<bool>,
    pub vfs_used_is_size: Option<bool>,
    pub vfs_write_back: Option<String>,
    pub vfs_write_wait: Option<String>,
    pub write_back_cache: Option<bool>,

    pub dir_cache_time: Option<String>,
    pub poll_interval: Option<String>,
    pub attr_timeout: Option<String>,

    pub async_read: Option<bool>,
    pub direct_io: Option<bool>,
    pub read_only: Option<bool>,
    pub no_checksum: Option<bool>,
    pub no_modtime: Option<bool>,
    pub no_seek: Option<bool>,
    pub max_read_ahead: Option<String>,
    pub fuse_flag: Option<Vec<String>>,
    pub option: Option<Vec<String>>,
    pub debug_fuse: Option<bool>,

    pub log_level: Option<LogLevel>,
    pub log_file: Option<String>,
}

impl MountPoint {
    pub fn is_enabled(&self) -> bool {
        self.enabled.unwrap_or(true)
    }

    pub fn mount_command(&self, environment: &Environment) -> Command {
        let mut cmd = Command::new(expand_home(&environment.rclone_bin()));

        cmd.arg("mount")
            .arg(expand_home(&self.remote))
            .arg(expand_home(&self.mount_dir));

        if let Some(true) = self.allow_other { cmd.arg("--allow-other"); }
        if let Some(true) = self.allow_non_empty { cmd.arg("--allow-non-empty"); }
        if let Some(true) = self.allow_root { cmd.arg("--allow-root"); }
        if let Some(true) = self.default_permissions { cmd.arg("--default-permissions"); }
        if let Some(true) = self.debug_fuse { cmd.arg("--debug-fuse"); }
        if let Some(true) = self.read_only { cmd.arg("--read-only"); }
        if let Some(true) = self.no_checksum { cmd.arg("--no-checksum"); }
        if let Some(true) = self.no_modtime { cmd.arg("--no-modtime"); }
        if let Some(true) = self.no_seek { cmd.arg("--no-seek"); }
        if let Some(true) = self.write_back_cache { cmd.arg("--write-back-cache"); }
        if let Some(true) = self.direct_io { cmd.arg("--direct-io"); }
        if let Some(true) = self.mount_case_insensitive { cmd.arg("--mount-case-insensitive"); }
        if let Some(true) = self.vfs_block_norm_dupes { cmd.arg("--vfs-block-norm-dupes"); }
        if let Some(true) = self.vfs_case_insensitive { cmd.arg("--vfs-case-insensitive"); }
        if let Some(true) = self.vfs_fast_fingerprint { cmd.arg("--vfs-fast-fingerprint"); }
        if let Some(true) = self.vfs_links { cmd.arg("--vfs-links"); }
        if let Some(true) = self.vfs_refresh { cmd.arg("--vfs-refresh"); }
        if let Some(true) = self.vfs_used_is_size { cmd.arg("--vfs-used-is-size"); }

        if let Some(async_read) = self.async_read {
            cmd.arg("--async-read").arg(async_read.to_string());
        }
        if let Some(uid) = self.uid { cmd.arg("--uid").arg(uid.to_string()); }
        if let Some(gid) = self.gid { cmd.arg("--gid").arg(gid.to_string()); }
        if let Some(umask) = self.umask { cmd.arg("--umask").arg(umask.to_string()); }
        if let Some(ref perms) = self.dir_perms { cmd.arg("--dir-perms").arg(perms); }
        if let Some(ref perms) = self.file_perms { cmd.arg("--file-perms").arg(perms); }
        if let Some(ref perms) = self.link_perms { cmd.arg("--link-perms").arg(perms); }
        if let Some(ref name) = self.devname { cmd.arg("--devname").arg(name); }

		if let Some(ref val) = self.cache_dir { cmd.arg("--cache-dir").arg(val); }
        if let Some(ref mode) = self.vfs_cache_mode {
            let mode_str = to_string(mode).unwrap().to_lowercase();
            cmd.arg("--vfs-cache-mode").arg(mode_str);
        }
        if let Some(ref val) = self.vfs_cache_max_age { cmd.arg("--vfs-cache-max-age").arg(val); }
        if let Some(ref val) = self.vfs_cache_max_size { cmd.arg("--vfs-cache-max-size").arg(val); }
        if let Some(ref val) = self.vfs_cache_min_free_space { cmd.arg("--vfs-cache-min-free-space").arg(val); }
        if let Some(ref val) = self.vfs_cache_poll_interval { cmd.arg("--vfs-cache-poll-interval").arg(val); }
        if let Some(ref val) = self.buffer_size { cmd.arg("--buffer-size").arg(val); }
        if let Some(ref val) = self.vfs_disk_space_total_size { cmd.arg("--vfs-disk-space-total-size").arg(val); }
        if let Some(ref val) = self.vfs_metadata_extension { cmd.arg("--vfs-metadata-extension").arg(val); }
        if let Some(ref val) = self.vfs_read_ahead { cmd.arg("--vfs-read-ahead").arg(val); }
        if let Some(ref val) = self.vfs_read_chunk_size { cmd.arg("--vfs-read-chunk-size").arg(val); }
        if let Some(ref val) = self.vfs_read_chunk_size_limit { cmd.arg("--vfs-read-chunk-size-limit").arg(val); }
        if let Some(streams) = self.vfs_read_chunk_streams { cmd.arg("--vfs-read-chunk-streams").arg(streams.to_string()); }
        if let Some(ref val) = self.vfs_read_wait { cmd.arg("--vfs-read-wait").arg(val); }
        if let Some(ref val) = self.vfs_write_back { cmd.arg("--vfs-write-back").arg(val); }
        if let Some(ref val) = self.vfs_write_wait { cmd.arg("--vfs-write-wait").arg(val); }
        if let Some(ref val) = self.dir_cache_time { cmd.arg("--dir-cache-time").arg(val); }
        if let Some(ref val) = self.poll_interval { cmd.arg("--poll-interval").arg(val); }
        if let Some(ref val) = self.attr_timeout { cmd.arg("--attr-timeout").arg(val); }
        if let Some(ref val) = self.max_read_ahead { cmd.arg("--max-read-ahead").arg(val); }

        if let Some(flags) = &self.fuse_flag {
            for f in flags { cmd.arg("--fuse-flag").arg(f); }
        }
        if let Some(opts) = &self.option {
            for o in opts { cmd.arg("-o").arg(o); }
        }

        if let Some(ref level) = self.log_level {
            let level_str = to_string(level).unwrap().to_uppercase();
            cmd.arg("--log-level").arg(level_str);
        }
        if let Some(ref file) = self.log_file { cmd.arg("--log-file").arg(expand_home(file)); }

        cmd.env("RCLONE_CONFIG", expand_home(environment.rclone_conf()));

        cmd
    }

    pub fn unmount_command(&self, environment: &Environment) -> Command {
        let mut cmd = Command::new(expand_home(environment.fusermount_bin()));
        cmd.arg("-u").arg(&self.mount_dir);
        cmd
    }
}





#[derive(Debug, Deserialize)]
pub struct Config {
    pub environment: Option<Environment>,
    pub mountpoints: HashMap<String, MountPoint>,
}