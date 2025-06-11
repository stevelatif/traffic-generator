# Traffic Runner

A simple wrapper to make CLI traffic generation tools appear from multiple
IP/MAC addresses. 

## Description

If you are testing your network or protocol and want traffic to appear from 
multiple IP/MAC addresses this tool will make each connection appear from a
different IP/MAC. 


## Getting Started

### Dependencies

* Builds and runs on Linux 2025
* Built from Rust 2024 
* Requires smbclient
* 
	Needs a wide open samba configuration smb.conf:
	```
  [global]
     workgroup = WORKGROUP
     server string = Samba Server
     netbios name = SAMBA_SERVER
     map to guest = Bad User
     dns proxy = no

  [public]
     path = /srv/samba/public
     writable = yes
     browsable = yes
     guest ok = yes
     read only = no
     create mask = 0777
     directory mask = 0777
	```

### Installing

* Build from source

	`cargo build`

### Executing program

* Single client:

	`sudo ./traffic-runner -a 192.168.56.20 -f test_file.zero -i enp16s0f0 -n foo -b 192.168.56.40 -e 192.168.56.40  -c 24`
	
* Fifty Clients:

	`sudo ./traffic-runner -a 192.168.56.20 -f test_file.zero -i enp16s0f0 -n foo -b 192.168.56.40 -e 192.168.56.90  -c 24`

## Help

	* Do not run in production
	* Use a dedicated network

## Authors

	Steve Latif
	[@stevelatif@gmail.com]

## Version History

* 0.1
    * Initial Release

## License

This project is licensed under the APACHE V2 License - see the LICENSE file for details

