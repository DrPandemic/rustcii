# rustcii
Simple image to ascii image.

<img src="https://user-images.githubusercontent.com/3250155/54496441-bab9b280-48c5-11e9-9b1b-904c0052ccf9.png" width="450">

<img src="https://user-images.githubusercontent.com/3250155/54077544-e9150d80-4287-11e9-9ed6-85797f09d573.png" width="450">

## How to use it?
```
USAGE:
    rustcii [FLAGS] [OPTIONS] <SIZE> <INPUT>
FLAGS:
    -h, --help       Prints help information
    -t               Renders in the terminal
    -V, --version    Prints version information
OPTIONS:
    -o, --output <FILE>    Sets the output file
ARGS:
    <SIZE>     Sets the tile size
    <INPUT>    Sets the input file to use
```
### Examples
`rustcii -o output.jpg 12 input.jpg`

and

`rustcii -t input.jpg 12`

- The `SIZE` is the size of a tile.
- `-t` outputs the result on the terminal.

## Supported formats
Every supported format in [piston's image library](https://github.com/PistonDevelopers/image#21-supported-image-formats) should be available.
