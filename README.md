# jetgpio-sys

This crate provides a low-level Rust interface to the [JETGPIO](https://github.com/Rubberazer/JETGPIO) package. That package provides memory-mapped (not file-based) access to NvidiaⓇ Jetson Nano™ GPIO interface. On standard installations, this requires root access.

## Limitations

This crate currently only supports the Jetson Nano. While the underlying library supports Orin as well, this is currently not supported for three reasons:

- the Orin support is currently in beta for the underlying library;
- the underlying library does build-time detection in a way that requires root access;
- the maintainer of this crate does not own a Jetson Orin.

However, contributions adding Orin support in a way that doesn't Nano support and doesn't require root at build-time are welcome.

At run-time, root access is required in standard configurations, to be able to directly access the memory that controls the GPIO. Only Linux is supported by this crate.

## Licence

This crate is published under the [UNLICENSE](LICENSE), just as the underlying library.
