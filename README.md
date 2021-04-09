# rsize
`rsize` is a little command line utillity to resize jpg and pngs concurrently.
Used threads can be specified, see usage.

## Usage
```sh
USAGE:
    rsize [OPTIONS]

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -h <height>          desired height [default: 1080]
    -s, --src <FILEs>    Resizes a single file or multiple by applying a directory [default: ./]
    -t <threads>         maximum count of threads [default: 16]
    -w <width>           desired width [default: 1920]
```
