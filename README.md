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

Download release and put it somewhere in your $PATH, for example:

```
wget https://gitlab.com/nxnjz/masssnd/uploads/d5716a43bc9ef6bd86cd79464423e14c/masssnd-linux-amd64.tar.gz -O masssnd.tar.gz
tar -xzf masssnd.tar.gz && rm masssnd.tar.gz
chmod +x masssnd
mv masssnd /usr/local/bin
```

Or to install from source using [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html): 

```
git clone https://gitlab.com/nxnjz/masssnd.git
cd masssnd
cargo install --path .
```

To update from source:

```
git pull
cargo install --path . --force
```

