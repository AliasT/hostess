use std::net::IpAddr;


#[derive(Debug)]
pub struct Hostname {
  pub domain: Option<String>,
  pub ip: Option<IpAddr>,
  pub enabled: bool,
  pub comment: String,
  pub valid: bool,
  // TODO: ip_v6: bool
}
