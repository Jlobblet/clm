# clm

Like [coreutils cut](https://www.gnu.org/software/coreutils/manual/html_node/The-cut-command.html) but slightly different.
Written in Rust.

```
clm 0.1.0

USAGE:
    clm [OPTIONS] <--field <field>|--col-name <col-name>>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --col-name <col-name>      
    -d, --delimiter <delimiter>     [default: \s+]
    -f, --field <field>
        --filter <filter>
```