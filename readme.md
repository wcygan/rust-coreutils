# rust-coreutils

A Rust implementation of gnu-coreutils programs (https://www.gnu.org/software/coreutils/manual/coreutils.html)

## Installation via [Cargo](https://www.rust-lang.org/tools/install)

Install on any platform using `cargo install`:

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
    cat     Copies each file, or standard input if none are given, to standard output
    date    Print the system date and time
    echo    Echos the provided text
    head    Prints the first part of each file
    help    Print this message or the help of the given subcommand(s)
    ls      List the contents of a directory
    nl      Prepends line numbers to the input files and prints them to stdout
    tree    Shows a file tree of a directory
    wc      Print newline, word, and byte counts
    yell    Echos the provided text in uppercase
    yes     Repeats the provided text until interrupted                         
```

You can execute a subcommand like `tree` in the following way:

```
$ rcu tree src
src
├── command       
│   ├── cat       
│   │   ├── lib.rs
│   │   └── mod.rs
│   ├── date      
│   │   ├── lib.rs
│   │   └── mod.rs
│   ├── echo      
│   │   ├── lib.rs
│   │   └── mod.rs
│   ├── head      
│   │   ├── lib.rs
│   │   └── mod.rs
│   ├── ls        
│   │   ├── lib.rs
│   │   └── mod.rs
│   ├── mod.rs    
│   ├── nl        
│   │   ├── lib.rs
│   │   └── mod.rs
│   ├── tree      
│   │   ├── lib.rs
│   │   └── mod.rs
│   ├── wc        
│   │   ├── lib.rs
│   │   └── mod.rs
│   ├── yell
│   │   ├── lib.rs
│   │   └── mod.rs
│   └── yes
│       ├── lib.rs
│       └── mod.rs
└── main.rs
```