```bash
$ factos --help
```
```bash
factos 0.1.0
remy2019 <remy2019@gmail.com>
Fair and clear time-originated sequence generator

USAGE:
    factos [OPTIONS] --length <length> --time <time>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -l, --length <length>    Input length of sequence
    -p, --phrase <phrase>    Optional phrase to make sequence unique
    -s, --start <start>      Set the starting number of sequence::default = 0 [default: 0]
    -t, --time <time>        Input datetime in KST as YYYY-MM-DD-HH-MM
```

### Example
```bash
$ factos --time=2025-08-18-10-00 --phrase=YLS2025-2 --start=0 --length=10

```
```bash

[9, 7, 1, 0, 3, 8, 2, 5, 6, 4]
```
