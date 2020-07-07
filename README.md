## Usage

```
USAGE:
    masssnd [OPTIONS] --input <FILE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --input <FILE>     Sets the input file to use.
                           Can contain both single IPv4 addresses and CIDR notations
    -t, --threads <INT>    Numbers of threads [default: 40]
```

## Dependencies

* dig. (Package "dnsutils" on Debian)

## Installation 

Download release and put it somewhere in your $PATH. 

Or to install from source using [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html): 

```
git clone https://gitlab.com/nxnjz/masssnd.git
cd masssnd
cargo install --path .
```


