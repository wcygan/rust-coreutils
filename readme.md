# rust-coreutils

A Rust implementation of gnu-coreutils programs (https://www.gnu.org/software/coreutils/manual/coreutils.html)

## Installation via cargo

Install on any platform using Cargo:

```
$ cargo install --git https://github.com/wcygan/rust-coreutils
```

## How to run

Once the binary is installed, use the `rcu` program like so:

```
$ rcu
rust-coreutils 0.1.0
Will C. <wcygan.io@gmail.com>
A Rust implementation of gnu-coreutils programs

USAGE:
    rcu.exe <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    date    Print the system date and time
    echo    Echos the provided text
    help    Print this message or the help of the given subcommand(s)
    ls      Echos the provided text
    tree    Shows a file tree of a directory
    wc      Print newline, word, and byte counts
    yes     Repeats the provided text until interrupted
```

You can execute a subcommand like `tree` in the following way:

```
$ rcu tree src
src
├── command
│   ├── date
│   │   ├── lib.rs
│   │   └── mod.rs
│   ├── echo
│   │   ├── lib.rs
│   │   └── mod.rs
│   ├── ls
│   │   ├── lib.rs
│   │   └── mod.rs
│   ├── mod.rs
│   ├── tree
│   │   ├── lib.rs
│   │   └── mod.rs
│   ├── wc
│   │   ├── lib.rs
│   │   └── mod.rs
│   └── yes
│       ├── lib.rs
│       └── mod.rs
└── main.rs
```