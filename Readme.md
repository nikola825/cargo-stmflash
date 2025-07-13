A **very** basic wrapper around STM32 Cube programmer to facilitate DFU flashing to STM32 chips.
In most cases, it's better to either use [cargo-dfu](https://github.com/dfu-rs/cargo-dfu) or [dfu-util](https://dfu-util.sourceforge.net/).

I found them to be slower than STM32 Cube in some cases, so I made this wrapper to call cube directly from cargo.
A lot of parameters are hardcoded because I don't need to change them in my wokflow.