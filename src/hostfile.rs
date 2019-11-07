// borrowed from
// https://github.com/cbednarski/hostess/blob/master/hostfile.go

use super::hostlist;
use super::hostname;

use pad::*;

use std::cmp;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::IpAddr;

#[derive(Debug)]
pub struct Hostfile {
  path: String,
  hosts: hostlist::Hostlist,
}

impl Hostfile {
  fn new(path: &str) -> Hostfile {
    let file = File::open(path).unwrap();
    let file_reader = BufReader::new(file);
    let mut hosts: hostlist::Hostlist = Vec::new();

    for line in file_reader.lines() {
      hosts.append(&mut Hostfile::parse_line(line.unwrap()));
    }

    let hostfile = Hostfile {
      path: String::from(path),
      hosts,
    };

    hostfile
  }

  pub fn default() -> Hostfile {
    let default_path = if cfg!(target_os = "windows") {
      "C:/Windows/System32/drivers/etc"
    } else {
      "/etc/hosts"
    };
    Hostfile::new(default_path)
  }

  /// append item to hosts
  pub fn append(&mut self, domain: String, ip: &str) {
    let host = hostname::Hostname {
      ip: ip.parse().expect("wrong ip format"),
      domain,
      enabled: true,
      comment: String::from(""),
    };

    self.hosts.push(host);
  }

  /// remove item from hosts
  pub fn remove() {
    unimplemented!();
  }

  // enable item
  pub fn on(&mut self, domain: String) -> &Self {
    for host in &mut self.hosts {
      if host.domain == domain {
        host.enabled = true;
      }
    }
    self
  }

  // disable item
  pub fn off(&mut self, domain: String) -> &Self {
    for host in &mut self.hosts {
      if host.domain == domain {
        host.enabled = false;
      }
    }

    self
  }

  //
  /// save write contents to hosts file
  /// this action remove all comments currently
  /// TODO: keep action
  pub fn save(&self) {
    let mut result = String::new();

    for host in &self.hosts {
      let is_comment = if host.enabled { "" } else { "# " };
      let line = format!("{}{} {}\n", is_comment, host.ip.to_string(), host.domain);
      result.push_str(line.as_str());
    }

    fs::write(&self.path, result).unwrap();
  }

  // output hosts
  pub fn format(&self) {
    let mut max_domain_len = 0;
    let mut max_ip_len = 0;

    // compute max len for output
    for host in &self.hosts {
      max_domain_len = cmp::max(host.domain.len(), max_domain_len);
      max_ip_len = cmp::max(host.ip.to_string().len(), max_ip_len);
    }

    for host in &self.hosts {
      let status = if host.enabled { "(on)" } else { "(off)" };
      println!(
        "{} -> {} {}",
        host.domain.pad_to_width(max_domain_len),
        host.ip.to_string().pad_to_width(max_ip_len),
        status
      );
    }
  }

  fn parse_line(mut line: String) -> hostlist::Hostlist {
    let mut hostlist: hostlist::Hostlist = Vec::new();
    let mut enabled = true;

    // comment
    if line.starts_with("#") {
      enabled = false;
      line.remove(0);
    }

    // trim whitespaces
    line = line.trim().to_string();

    // Parse other # for actual comments
    let line: Vec<&str> = line.split("#").collect();

    // host 条目
    let content: &str = line.get(0).unwrap();
    // 评论内容
    let comment: &str = match line.get(1) {
      Some(comment) => comment,
      None => &"",
    };

    let mut words = content.split_whitespace();

    // force now!
    let ip = match words.next().unwrap_or_default().parse() {
      Ok(IpAddr::V4(ip)) => Some(IpAddr::V4(ip)),
      Ok(IpAddr::V6(ip)) => Some(IpAddr::V6(ip)),
      _ => None,
    };

    if let Some(ip) = ip {
      for (_, domain) in words.enumerate() {
        let id = hostname::Hostname {
          ip,
          enabled,
          domain: domain.to_string(),
          comment: String::from(comment),
        };
        hostlist.push(id);
      }
    }

    hostlist
  }
}
