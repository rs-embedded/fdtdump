# fdtdump

[![crates.io](https://img.shields.io/crates/v/fdtdump)](https://crates.io/crates/fdtdump)
[![downloads](https://img.shields.io/crates/d/fdtdump.svg)](https://crates.io/crates/fdtdump)
[![docs.rs](https://docs.rs/fdtdump/badge.svg)](https://docs.rs/fdtdump/)
[![master](https://github.com/rs-embedded/fdtdump/workflows/Build/badge.svg?branch=master)](https://github.com/rs-embedded/fdtdump/actions)
[![coveralls.io](https://coveralls.io/repos/github/rs-embedded/fdtdump/badge.svg)](https://coveralls.io/github/rs-embedded/fdtdump)

A rust version of fdtdump - a tool for printing flattened device trees.

## Installation

Simply run the following to build and install:

cargo install fdtdump

## Usage 

```
USAGE:
    fdtdump <dtb-file>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <dtb-file>    Path to dtb file
```

## Example

```
$ echo '/dts-v1/; / { prop = "hello"; };' | dtc > dtb-file
$ cargo run dtb-file
/dts-v1/;
// magic:		0xd00dfeed
// totalsize:		0x61 (97)
// off_dt_struct:	0x38
// off_dt_strings:	0x5c
// version:		0x11
// boot_cpuid_phys:	0x0
// last_comp_version:	16
// boot_cpuid_phys:	0x0
// size_dt_strings:	0x5
// size_dt_struct:	0x24

/ {
    prop = "hello";
};
```
