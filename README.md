# rclone-automount

`rclone-automount` mounts multiple rclone remotes automatically based on a single `TOML` configuration file.  
It starts all configured mounts when launched and unmounts them cleanly on exit.

---

## Installation

Download the `.deb` package or binary from the [Releases](../../releases) section and install:

```bash
sudo dpkg -i rclone-automount_<version>_amd64.deb
```

The binary will be installed in `/usr/bin/rclone-automount`.

---

## Configuration

Create a configuration file (e.g., `/etc/rclone-automount/config.toml`) with the environment and mount points you need.

### Example

```toml
[environment]
rclone_bin = "/usr/bin/rclone"
rclone_conf = "~/.config/rclone/rclone.conf"
fusermount_bin = "/usr/bin/fusermount"

[mountpoints.remote1]
enabled = true
remote = "gdrive:myfolder"
mount_dir = "/mnt/remote1"
allow_other = true
vfs_cache_mode = "full"
dir_cache_time = "10m"

[mountpoints.remote2]
enabled = true
remote = "dropbox:data"
mount_dir = "/mnt/remote2"
read_only = true
vfs_cache_mode = "minimal"
```

### Supported options

* **Environment:**
  `rclone_bin`, `rclone_conf`, `fusermount_bin`
* **Mount point:**
  `enabled`, `remote`, `mount_dir`,
  `allow_other`, `allow_non_empty`, `allow_root`,
  `default_permissions`, `uid`, `gid`, `umask`,
  `dir_perms`, `file_perms`, `link_perms`, `devname`,
  `mount_case_insensitive`, `cache_dir`,
  `vfs_cache_mode`, `vfs_cache_max_age`, `vfs_cache_max_size`, `vfs_cache_min_free_space`,
  `vfs_cache_poll_interval`, `buffer_size`, `vfs_block_norm_dupes`,
  `vfs_case_insensitive`, `vfs_fast_fingerprint`, `vfs_links`, `vfs_metadata_extension`,
  `vfs_read_ahead`, `vfs_read_chunk_size`, `vfs_read_chunk_size_limit`,
  `vfs_read_chunk_streams`, `vfs_read_wait`, `vfs_refresh`,
  `vfs_used_is_size`, `vfs_write_back`, `vfs_write_wait`, `write_back_cache`,
  `dir_cache_time`, `poll_interval`, `attr_timeout`,
  `async_read`, `direct_io`, `read_only`,
  `no_checksum`, `no_modtime`, `no_seek`, `max_read_ahead`,
  `fuse_flag`, `option`, `debug_fuse`,
  `log_level`, `log_file`

---

## Usage

Run the binary specifying the config file:

```bash
sudo rclone-automount /etc/rclone-automount/config.toml
```

---

## Systemd service example

Create `/etc/systemd/system/rclone-automount.service`:

```ini
[Unit]
Description=Rclone Automount Service
After=network-online.target

[Service]
Type=simple
User=myuser
ExecStart=/usr/bin/rclone-automount /path/to/rclone-automount/config.toml
Restart=on-failure

[Install]
WantedBy=multi-user.target
```

Enable and start:

```bash
sudo systemctl daemon-reload
sudo systemctl enable rclone-automount
sudo systemctl start rclone-automount
```

---

## License

MIT – see [LICENSE](LICENSE)

```