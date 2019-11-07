use std::net::IpAddr;

#[derive(Debug)]
pub struct Hostname {
  pub domain: String,
  pub ip: IpAddr,
  pub enabled: bool,
  pub comment: String,
  // TODO: ip_v6: bool
}
