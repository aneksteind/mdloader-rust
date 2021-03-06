# mdloader-rust

A rust port of the [Massdrop Firmware Loader](https://github.com/Massdrop/mdloader) - for CTRL / ALT / SHIFT / Rocketeer keyboards.

The rust in the [initial commit](https://github.com/aneksteind/mdloader-rust/commit/a20b8854fa3892daf9dd0319e7823de15a1c3371) of this project was generated, in part, by [c2rust](https://github.com/immunant/c2rust), with a few exceptions:

- [`incbin`](https://github.com/graphitemaster/incbin) was replaced with [lazy-static-include](https://docs.rs/lazy-static-include/latest/lazy_static_include/index.html) 
- the `applet_data` and `applet_size` generated by `incbin` [here](https://github.com/Massdrop/mdloader/blob/e4f977416994f54ca3f8d2e72f2b225d52f7c42e/mdloader_common.c#L23) were replaced with `applet` and `applet.len()` using `lazy-static-include`, respectively
- ATMEL-specific code was placed into the `atmel` module, under an ATMEL-specific license.

## Usage

_Note: permissions may need to be given to the_ `/dev` _port the keyboard uses, which only appears when it is in DFU mode_ 

```
$ cargo build
$ cargo run
```

## Caveats

This port hasn't been tested on Windows, which the original `mdloader` supports -- only Linux.

## Contribution

Contribution is welcome, just open a pull request.
