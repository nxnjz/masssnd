use std::fs;
use std::net::Ipv4Addr;
use std::process;
use std::str::FromStr;
use std::thread;

use cidr::{Cidr, Ipv4Cidr};
use clap::{App, Arg};
use crossbeam::channel;

fn main() {
    let matches = App::new("masssnd")
        .version("0.0.1")
        .author("")
        .about("Mass reverse DNS lookups using dig")
        .arg(
            Arg::with_name("threads")
                .short("t")
                .long("threads")
                .value_name("INT")
                .help("Numbers of threads")
                .default_value("40")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("infile")
                .short("i")
                .long("input")
                .help(
                    "Sets the input file to use.
Can contain both single IPv4 addresses and CIDR notations",
                )
                .required(true)
                .takes_value(true)
                .value_name("FILE"),
        )
        .get_matches();

    let input = fs::read_to_string(matches.value_of("infile").unwrap()).unwrap();
    let mut thr_cnt: usize = matches.value_of("threads").unwrap().parse().unwrap();
    let mut addrs: Vec<Ipv4Addr> = Vec::new();
    for line in input.lines().filter(|l| l.contains('.')) {
        if line.contains('/') {
            addrs.append(&mut (Ipv4Cidr::from_str(line).unwrap().iter().collect()));
        } else {
            addrs.push(Ipv4Addr::from_str(line).unwrap());
        }
    }
    addrs.sort();
    addrs.dedup();
    eprintln!("Processing a total of {} addresses", addrs.len());

    if addrs.len() < thr_cnt {
        thr_cnt = addrs.len();
    }

    let (tx, rx) = channel::unbounded();
    let mut threads = Vec::new();
    for _ in 0..thr_cnt {
        let rxt = rx.clone();
        threads.push(thread::spawn(move || thread(rxt)));
    }
    while !addrs.is_empty() {
        tx.send(addrs.pop().unwrap());
    }
    drop(tx);

    for t in threads.into_iter() {
        t.join();
    }
}

fn thread(rx: channel::Receiver<Ipv4Addr>) {
    while rx.is_empty() {
        thread::yield_now();
    }
    for ip in rx.iter() {
        process::Command::new("dig")
            .arg("+short")
            .arg("-x")
            .arg(&format!("{}", ip))
            .status();
        //println!("{}", ip);
    }
}
