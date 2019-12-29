// borrowed from
// https://github.com/cbednarski/hostess/blob/master/hostfile.go

use super::hostlist;
use super::hostname;

use pad::*;
use std::cmp;
use std::error;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use yansi::Paint;

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
  pub fn append(&mut self, domain: String, ip: &str) -> Result<(), Box<dyn error::Error>> {
    let host = hostname::Hostname {
      ip: Some(ip.parse()?),
      domain: Some(domain),
      valid: true,
      enabled: true,
      comment: String::from(""),
    };

    self.hosts.push(host);
    Ok(())
  }

  /// remove item from hosts
  pub fn remove() {
    unimplemented!();
  }

  // enable item
  pub fn on(&mut self, domain: String) -> &Self {
    for host in &mut self.hosts {
      if host.domain == Some(domain.clone()) {
        host.enabled = true;
      }
    }
    self
  }

  // disable item
  pub fn off(&mut self, domain: String) -> &Self {
    for host in &mut self.hosts {
      if host.domain == Some(domain.clone()) {
        host.enabled = false;
      }
    }

    self
  }

  /// save write contents to hosts file
  /// this action remove all comments currently
  /// TODO: keep action
  pub fn save(&self) {
    let mut result = String::new();

    for host in &self.hosts {
      let is_comment = if host.enabled { "" } else { "# " };
      let line;
      if host.valid {
        line = format!(
          "{}{} {}\n",
          is_comment,
          host.ip.unwrap(),
          host.domain.as_ref().unwrap()
        );
      } else {
        line = format!("{}\n", host.comment.clone());
      }

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
      if let Some(ip) = host.ip {
        max_domain_len = cmp::max(host.domain.as_ref().unwrap().len(), max_domain_len);
        max_ip_len = cmp::max(ip.to_string().len(), max_ip_len);
      }
    }

    for host in &self.hosts {
      let status = if host.enabled {
        format!("({})", Paint::green("on"))
      } else {
        format!("({})", Paint::red("off"))
      };
      if let Some(ip) = host.ip {
        println!(
          "{} -> {} {}",
          host.domain.as_ref().unwrap().pad_to_width(max_domain_len),
          ip.to_string().pad_to_width(max_ip_len),
          status
        );
      }
    }
  }

  fn parse_line(source_string: String) -> hostlist::Hostlist {
    let mut hostlist: hostlist::Hostlist = Vec::new();
    let mut enabled = true;

    // comment
    let mut source = source_string.clone();

    // 去除首尾空白
    source = source.trim().to_string();
    if source.starts_with("#") {
      enabled = false;
      source.remove(0);
    }

    // Parse other # for actual comments
    let line: Vec<&str> = source.split("#").collect();

    // host 条目
    let content: &str = line.get(0).unwrap();
    // 评论内容
    let comment: &str = line.get(1).unwrap_or(&"");
    let mut words = content.split_whitespace();

    // force now!
    let ip = words.next().unwrap_or_default().parse();

    if let Ok(ip) = ip {
      for (_, domain) in words.enumerate() {
        let id = hostname::Hostname {
          ip: Some(ip),
          enabled,
          valid: true,
          domain: Some(domain.to_string()),
          comment: String::from(comment),
        };
        hostlist.push(id);
      }
    } else {
      // 如果parse失败，将整行当作注释直接存储。
      let id = hostname::Hostname {
        ip: None,
        domain: None,
        enabled: false,
        comment: source_string,
        valid: false,
      };
      hostlist.push(id);
    }

    hostlist
  }
}
