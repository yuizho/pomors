# pomors
A simple CLI pomodoro timer written in Rust 🍅
![pomors_demo](pomors.gif)

# Getting Started
If you haven't installed cargo, follow the procedure below to install it.

https://doc.rust-lang.org/cargo/getting-started/installation.html

```
$ cargo install pomors
$ pomors
```

## Usage
```
USAGE:
    pomors [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -l, --long-break-sec <long_break_sec>       [default: 1200]
    -s, --short-break-sec <short_break_sec>     [default: 300]
    -w, --work-sec <work_sec>                   [default: 1500]
```

## License
MIT
