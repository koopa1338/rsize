# rsize
`rsize` is a little command line utillity to resize jpg and pngs concurrently
by iterating in parallel over the images using
[rayon](https://github.com/rayon-rs/rayon).

## Performance
Before I used a little bash script that checked if the image had not the
correct dimensions and resized it using the mogrify command to resize it
inplace. The performance was quite poor and for about 180 images it took about
10s on my CPU. With rsize the same operation takes about 1.4s on the same
hardware.

## Usage
```sh
USAGE:
    rsize [OPTIONS]

OPTIONS:
    -h, --help               Print help information
        --height <HEIGHT>    [default: 1080]
    -i, --ignore-aspect
    -r, --recursive
    -s, --src <SRC>          [default: ./]
    -V, --version            Print version information
        --width <WIDTH>      [default: 1920]
```
