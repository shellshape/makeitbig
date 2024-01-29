# makeitbig

A tiny CLI tool to generate files with random data of a given size.

## Usage

```
$ makeitbig --help
Tiny CLI tool to generate random files of given sizes

Usage: makeitbig [OPTIONS] --size <SIZE>

Options:
  -o, --output <OUTPUT>
          File to write output to. If not specified, output is written to StdOut.

  -s, --size <SIZE>
          Size of the output.
          Format examples: 512, 2m, 8G, ...

      --chunk-size <CHUNK_SIZE>
          Size of each write buffer. Larger buffer size can improve write speed but increases memory consumption.
          Format examples: 512, 2m, 8G, ...
          
          [default: 4k]

  -z, --zeroed
          Write zero bytes instead of random data

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Install

You can either download the latest release builds form the [Releases page](https://github.com/zekroTJA/makeitbig/releases) or you can install it using cargo install.
```
cargo install --git https://github.com/shellshape/makeitbig
```
