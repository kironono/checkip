# `checkip`

[![version](https://img.shields.io/crates/v/checkip)](https://crates.io/crates/checkip)
[![license](https://img.shields.io/crates/l/checkip)](https://crates.io/crates/checkip)

Remote IP checker.

## Installation

cargo install:

```
cargo install checkip
```

## Usage

```
USAGE:
    checkip [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b, --binding <binding>    Binds server to the specified IP [default: 127.0.0.1]
    -p, --port <port>          Runs server on the specified port [default: 8080]
```

```
$ curl -XGET http://localhost:8080/
127.0.0.1
```

## License

`checkip` is distributed under the terms of the MIT license.

See the [LICENSE](LICENSE) files in this repository for more information.
