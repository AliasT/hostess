use std::net::IpAddr;
use std::cmp::{PartialEq};

#[derive(Debug, PartialEq)]
pub struct Hostname {
  pub domain: Option<String>,
  pub ip: Option<IpAddr>,
  pub enabled: bool,
  pub comment: String,
  pub valid: bool,
  // TODO: ip_v6: bool
}
