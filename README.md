# rsize
`rsize` is a little command line utillity to resize jpg and pngs concurrently
by iterating in parallel over the images using rayon.

## Performance
Before I used a little bash script that checked if the image had not the
correct dimensions and resized it using the mogrify command to resize it
inplace. The performance was quite poor and at about 180 images it took about 10s on a AMD Ryzen 7 4700U.
With rsize the same operation takes about 1.4s on the same hardware.

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
    -w <width>           desired width [default: 1920]
```
