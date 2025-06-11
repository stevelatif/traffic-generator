use clap::Arg;
use ipnet::Ipv4AddrRange;
use std::net::Ipv4Addr;
use tokio::process::Command;
use tokio::task;
use tokio::time::{Duration, sleep};

#[derive(Debug, Clone)]
struct LocalConfig {
    hosts: Ipv4AddrRange,
    count: usize,
    interface: String,
    cidr_suffix: String,
    base_namespace: String,
}

async fn set_up(config: LocalConfig) -> Result<(), Box<dyn std::error::Error>> {
    //let base_if = "mvlan".to_string();
    for (idx, ii) in config.hosts.enumerate() {
        // interface name cannont be longer than 16 characters
        // take the last two octects and merge  with the string `macvlan`
        // ie IP address 192.168.20.101 --> mvlan1
        // 192.168.20.102 --> mvlan2
        let ns = format!("{}{}", config.base_namespace, idx);
        let macvlan = format!("macvlan{}", idx);
        let cidr = ii.to_string() + "/" + &(config.cidr_suffix);
        let ns_output = Command::new("/usr/sbin/ip")
            .arg("netns")
            .arg("add")
            .arg(&ns)
            .output();
        let _ns_output = ns_output.await?;
        // println!(
        //     "link netns add exited with stdout: {:?} stderr: {:?}",
        //     str::from_utf8(&ns_output.stdout),
        //     str::from_utf8(&ns_output.stderr)
        // );

        //     ip link add $MV link $PARENT_IF type macvlan mode bridge
        let ip_link_output = Command::new("/usr/sbin/ip")
            .arg("link")
            .arg("add")
            .arg(&macvlan)
            .arg("link")
            .arg(&config.interface)
            .arg("type")
            .arg("macvlan")
            .arg("mode")
            .arg("bridge")
            .output();
        let _ip_link_output = ip_link_output.await?;
        // println!(
        //     "link ip link exited with stdout: {:?} stderr: {:?}",
        //     str::from_utf8(&ip_link_output.stdout),
        //     str::from_utf8(&ip_link_output.stderr)
        // );

        //     ip link set $MV netns $NS
        let ns_output = Command::new("/usr/sbin/ip")
            .arg("link")
            .arg("set")
            .arg(&macvlan)
            .arg("netns")
            .arg(&ns)
            .output();
        let _ns_output = ns_output.await?;
        // println!(
        //     "link netns add exited with stdout: {:?} stderr: {:?}",
        //     str::from_utf8(&ns_output.stdout),
        //     str::from_utf8(&ns_output.stderr)
        // );

        //     ip netns exec $NS ip addr add ${IP}/24 dev $MV
        //     ip link set $MV netns $NS
        let ns_output = Command::new("/usr/sbin/ip")
            .arg("netns")
            .arg("exec")
            .arg(&ns)
            .arg("ip")
            .arg("addr")
            .arg("add")
            .arg(cidr)
            .arg("dev")
            .arg(&macvlan)
            .output();
        let _ns_output = ns_output.await?;
        // println!(
        //     "link netns exec addr add exited with stdout: {:?} stderr: {:?}",
        //     str::from_utf8(&ns_output.stdout),
        //     str::from_utf8(&ns_output.stderr)
        // );

        //     ip netns exec $NS ip link set $MV up
        let ns_output = Command::new("/usr/sbin/ip")
            .arg("netns")
            .arg("exec")
            .arg(&ns)
            .arg("ip")
            .arg("link")
            .arg("set")
            .arg(&macvlan)
            .arg("up")
            .output();
        let _ns_output = ns_output.await?;
        // println!(
        //     "link netns exec link up exited with stdout: {:?} stderr: {:?}",
        //     str::from_utf8(&ns_output.stdout),
        //     str::from_utf8(&ns_output.stderr)
        // );

        //     ip netns exec $NS ip link set lo up
        let ns_output = Command::new("/usr/sbin/ip")
            .arg("netns")
            .arg("exec")
            .arg(&ns)
            .arg("ip")
            .arg("link")
            .arg("set")
            .arg("lo")
            .arg("up")
            .output();
        let _ns_output = ns_output.await?;
        // println!(
        //     "link netns exec link lo up exited with stdout: {:?} stderr: {:?}",
        //     str::from_utf8(&ns_output.stdout),
        //     str::from_utf8(&ns_output.stderr)
        // );

        //     ip netns exec $NS ip route add default dev $MV
        let ns_output = Command::new("/usr/sbin/ip")
            .arg("netns")
            .arg("exec")
            .arg(&ns)
            .arg("ip")
            .arg("route")
            .arg("add")
            .arg("default")
            .arg("dev")
            .arg(&macvlan)
            .output();
        let _ns_output = ns_output.await?;
        // println!(
        //     "link netns add default route exited with stdout: {:?} stderr: {:?}",
        //     str::from_utf8(&ns_output.stdout),
        //     str::from_utf8(&ns_output.stderr)
        // );
    }

    Ok(())
}

fn set_up_config(
    base_address: Ipv4Addr,
    end_address: Ipv4Addr,
    cidr_suffix: &str,
    interface: &str,
    base_namespace: &str,
) -> Result<LocalConfig, ()> {
    let hosts = Ipv4AddrRange::new(base_address, end_address);
    let (tcount, _) = hosts.size_hint();

    let config = LocalConfig {
        hosts,
        count: tcount,
        interface: interface.to_string(),
        cidr_suffix: cidr_suffix.to_string(),
        base_namespace: base_namespace.to_string(),
    };
    Ok(config)
}

async fn clean_up(config: LocalConfig) -> Result<(), Box<dyn std::error::Error>> {
    for (idx, _) in config.hosts.enumerate() {
	let nms = format!("{}{}", config.base_namespace, idx);
	let mvlans = format!("macvlan{}", idx);
	println!("deleting interface: {}", mvlans);
	
	let clean_up_output = Command::new("/usr/sbin/ip")
            .arg("netns")
            .arg("del")
            .arg(nms)
            .output();
	let _clean_up_output = clean_up_output.await?;
	// println!(
        //     "link del namespaces exited with stdout: {:?} stderr: {:?}",
        //     str::from_utf8(&clean_up_output.stdout),
        //     str::from_utf8(&clean_up_output.stderr)
	// );
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let matches = clap::Command::new("SMB Traffic Generation Tool")
        .version("0.1.0")
        .author("Steve Latif <stevelatif@gmail.com>")
        .about("Spawns concuurent SMB connections")
        .arg(
            Arg::new("smb_address")
                .short('a')
                .long("samba-address")
                .help("address of the SAMBA server"),
        )
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .help("file to download from the Samba server"),
        )
        .arg(
            Arg::new("interface")
                .short('i')
                .long("interace")
                .help("network interface to attach the virtual interfaces to"),
        )
        .arg(
            Arg::new("cidr_suffix")
                .short('c')
                .long("cidr-suffix")
                .help("network identifier for the number of bits "),
        )
        .arg(
            Arg::new("base_namespace")
                .short('n')
                .long("base_namespace")
                .help("basename for namespaces"),
        )
        .arg(
            Arg::new("base_address")
                .short('b')
                .long("base-address")
                .help(
                    "base IP address for the virtual interfaces. Given a \n\
		       base address of 10.0.1.0 and an end-address of\n\
		       10.0.1.3. The following addresses will be generated:\n\
		       10.0.1.1\n\
		       10.0.1.2\n\
		       10.0.1.3",
                ),
        )
        .arg(Arg::new("end_address").short('e').long("end-address").help(
            "end IP address for the virtual interfaces. Given a \n\
		       base address of 10.0.1.0 and an end-address of\n\
		       10.0.1.3 The following addresses will be generated:\n\
		       10.0.1.1\n\
		       10.0.1.2\n\
		       10.0.1.3",
        ))
        .get_matches();

    let a = matches.get_one::<String>("smb_address");
    let smb_address: Ipv4Addr = match a {
        None => {
            eprintln!("Need address of SAMBA server");
            std::process::exit(1);
        }
        Some(s) => {
            let n = s;
            n.parse::<Ipv4Addr>().expect("failed to get value")
        }
    };

    let f = matches.get_one::<String>("file");
    let file: &String = match f {
        None => {
            eprintln!("need file to download from server");
            std::process::exit(1);
        }
        Some(s) => (s) as _,
    };

    let i = matches.get_one::<String>("interface");
    let interface: &String = match i {
        None => {
            eprintln!("need local network interface");
            std::process::exit(1);
        }
        Some(s) => (s) as _,
    };

    let b = matches.get_one::<String>("base_address");
    let base_address: Ipv4Addr = match b {
        None => {
            eprintln!("Need base address for virtual interfaces");
            std::process::exit(1);
        }
        Some(s) => {
            let n = s;
            n.parse::<Ipv4Addr>().expect("failed to get value")
        }
    };

    let e = matches.get_one::<String>("end_address");
    let end_address: Ipv4Addr = match e {
        None => {
            eprintln!("Need end address for virtual interfaces");
            std::process::exit(1);
        }
        Some(s) => {
            let n = s;
            n.parse::<Ipv4Addr>().expect("failed to get value")
        }
    };

    let c = matches.get_one::<String>("cidr_suffix");
    let cidr_suffix: &String = match c {
        None => {
            eprintln!("cidr suffix for network address");
            std::process::exit(1);
        }
        Some(s) => (s) as _,
    };

    let n = matches.get_one::<String>("base_namespace");
    let base_namespace: &String = match n {
        None => {
            eprintln!("base namespace name");
            std::process::exit(1);
        }
        Some(s) => s as _,
    };

    let count = set_up_config(
        base_address,
        end_address,
        cidr_suffix,
        interface,
        base_namespace,
    );
    let local_config: LocalConfig = match count {
        Ok(n) => n,
        Err(e) => panic!("could not set up the configuration {:#?}", e),
    };

    let _res = set_up(local_config.clone()).await;
    sleep(Duration::from_millis(100)).await;
    spawn_task(local_config.clone(), smb_address, file).await;
    sleep(Duration::from_millis(100)).await;
    let _res = clean_up(local_config).await;
}




async fn spawn_task(config: LocalConfig,
		    smb_address: Ipv4Addr,
		    filename: &String) {
    let (tx, rx) = flume::bounded(10);
    for (idx, _ii) in config.hosts.enumerate() {
        let tx = tx.clone();
	
        let namespace_ii = format!("{}{}", config.base_namespace, idx);
        let add = format!("//{}/public/", smb_address);
	let ff = format!("get {}", filename);

        // Convert address string to Ipv4Addr
        task::spawn(async move {
            let output = Command::new("/usr/sbin/ip")
                .arg("netns")
                .arg("exec")
                .arg(namespace_ii)
                .arg("smbclient")
                .arg("-Uguest")
                .arg("-N")
                .arg(add)
                .arg(smb_address.to_string())
                .arg("-c")
                .arg(ff)
                .output()
                .await;

            match output {
                Ok(_out) => {
                    tx.send_async(0).await.unwrap();
                }
                Err(e) => {
                    eprintln!("could not format the command: {}", e);
                }
            }
        });
    }
    drop(tx);
    
    for ii in 0..config.count {
        let message = rx.recv().unwrap();
        println!("Task {ii} completed with output: {:?}", message);
    }
}

