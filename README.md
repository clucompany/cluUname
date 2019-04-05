# cluUname

[![Build Status](https://travis-ci.org/clucompany/cluUname.svg?branch=master)](https://travis-ci.org/clucompany/cluUname)
[![Apache licensed](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](./LICENSE)
[![crates.io](http://meritbadge.herokuapp.com/cluuname)](https://crates.io/crates/cluuname)
[![Documentation](https://docs.rs/cluuname/badge.svg)](https://docs.rs/cluuname)


Library for displaying information about the system. Implemented only for Linux.

# Print

```
extern crate cluuname;
use cluuname::uname;

fn main() {
	let uname = uname().unwrap();
	println!("{}", uname);
	//Linux cluComp 4.15.15-1-zen #1 ZEN SMP PREEMPT Sat Mar 31 23:59:18 UTC 2018 x86_64
}
```

# 2Print

```
extern crate cluuname;
use cluuname::uname;
use cluuname::UtsName;
use cluuname::build;

fn main() {
	let uname = uname().unwrap();
	nodename(uname);
	//NODENAME "R510"

	let custom_uname = build::linux_216_86();
	nodename(custom_uname);
	//NODENAME "cluComp"
}

fn nodename<T: UtsName>(uname: T) {
	println!("NODENAME {}", uname.display_nodename());
}
```

# CustomPrint

```
extern crate cluuname;
use cluuname::uname;
use cluuname::UtsName;

fn main() {
	let uname = uname().unwrap();

	let sysname = uname.display_sysname();
	let nodename = uname.display_nodename();
	let release = uname.display_release();
	let version = uname.display_version();
	let machine = uname.display_machine();

	println!("{} {} {} {} {}", sysname, nodename, release, version, machine);
	//Linux cluComp 4.15.15-1-zen #1 ZEN SMP PREEMPT Sat Mar 31 23:59:18 UTC 2018 x86_64
}
```

# Hash + Hash Version

```
extern crate cluuname;
use cluuname::uname;
use cluuname::UtsName;

fn main() {
		let uname = uname().unwrap();

		let machine_all_hash = uname.uname_hash();
		let machive_version_hash = uname.version_hash();

		println!("UNAME_HASH {}", machine_all_hash);
		//12821596144084292007
		println!("UNAME_V_HASH {}", machive_version_hash);
		//2978006705337010168
}
```

# CustomUname

```
#![feature(plugin)]
#![plugin(clucstr)]
extern crate cluuname;
use cluuname::build;

use std::ffi::CStr;

fn main() {
	let uname = build::custom(
		cstr!("Linux"),
		cstr!("cluComp"),
		cstr!("2.16-localhost"),
		cstr!("#1 SMP PREEMPT Sat Mar 31 23:59:18 UTC 2008"),
		cstr!("x86"),
	);
	println!("{}", uname);
	//Linux cluComp 2.16-localhost #1 SMP PREEMPT Sat Mar 31 23:59:18 UTC 2008 x86
}
```

# Flags
enable_domainname - Additional item `domainname`

[dependencies]
cluuname = { version = "*", features = ["enable_domainname"] }


# License

Copyright 2018 #UlinProject Денис Котляров

Licensed under the Apache License, Version 2.0
