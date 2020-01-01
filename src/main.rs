use hostess::hostfile;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "hostess")]

enum Opt {
    #[structopt(about = "List all hosts")]
    List {},

    #[structopt(about = "Enable a host")]
    On {
        #[structopt(name = "DOMAIN")]
        domain: String,
    },

    #[structopt(about = "Disable a host")]
    Off {
        #[structopt(name = "DOMAIN")]
        domain: String,
    },

    #[structopt(about = "Remove a host")]
    Rm {
        #[structopt(name = "DOMAIN")]
        domain: String,
    },

    #[structopt(about = "Add a host")]
    Add {
        // domain 和 ip的位置不能随意变动
        #[structopt(name = "DOMAIN")]
        domain: String,
        #[structopt(name = "IP", default_value = "127.0.0.1")]
        ip: String,
    },
}

fn main() {
    let matches = Opt::from_args();
    let mut hosts = hostfile::Hostfile::default();

    match matches {
        Opt::List {} => {
            hosts.format();
        }
        Opt::On { domain } => {
            hosts.on(domain);
        }
        Opt::Off { domain } => {
            hosts.off(domain);
        }
        Opt::Add { ip, domain } => {
            hosts.append(domain, &ip).unwrap();
        }
        Opt::Rm { domain } => {
            hosts.remove(domain);
        }
    }
}
