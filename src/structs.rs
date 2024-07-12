
use std::path::PathBuf;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SuitesVM {
  pub metadata: MetadataBlock,
  pub setup:    SetupBlock,
  pub machine:  MachineBlock,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetadataBlock {
  pub name: String,

  #[serde(default = "empty_string")]
  pub description: String,

  pub pocs: Vec<String>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct SetupBlock {
  // #[serde(default = "empty_string")]
  // pub boot_iso_url: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct MachineBlock {
  // #[serde(default = "empty_string")]
  // pub boot_iso_url: String,
}

/*
#[derive(Debug, Serialize, Deserialize)]
pub struct VMBlock {
  pub name: String,

  #[serde(default = "dev_null_pathbuf")]
  pub disk_image: PathBuf,
  #[serde(default = "zero_usize")]
  pub disk_image_gb: usize,

  #[serde(default = "empty_string")]
  pub disk_partuuid: String,

  #[serde(default = "false_bool")]
  pub mount_windows_virtio_iso: bool,

  pub ram_mb: usize,

  #[serde(default = "empty_vec_string")]
  pub addtl_args: Vec<String>,

  #[serde(default = "empty_string")]
  pub rdp_uname: String,
  #[serde(default = "empty_string")]
  pub rdp_pass: String,

  #[serde(default = "empty_vec_string")]
  pub addtl_rdp_args: Vec<String>,

}

*/


fn empty_string() -> String {
  String::new()
}

fn dev_null_pathbuf() -> PathBuf {
  PathBuf::from("/dev/null")
}

fn zero_usize() -> usize {
  0
}

fn false_bool() -> bool {
  false
}

fn empty_vec_string() -> Vec<String> {
  vec![]
}

