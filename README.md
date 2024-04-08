# jetgpio-sys

This crate provides a low-level Rust interface to the [JETGPIO](https://github.com/Rubberazer/JETGPIO) package. That package provides memory-mapped (not file-based) access to NvidiaⓇ Jetson Nano™ or Jetson Orin Nano™ GPIO interface. On standard installations, this requires root access.

By default, the `orin` feature is disabled and Jetson Nano™ implementation is built. If the `orin` feature is enabled, then Orin support is built instead (and Nano support is not).

Clang is required in the compilation process.

## Limitations

At run-time, root access is required in standard configurations, to be able to directly access the memory that controls the GPIO. Only Linux is supported by this crate.

The crate can be built to either work on Jetson Nano™ or Jetson Orin Nano™, but not both at the same time. Contributions to relax this limitation are welcome.

## Licence

This crate is published under the [UNLICENSE](LICENSE), just as the underlying library.

## Disclaimer

This crate or its author is not affiliated, associated, authorized, endorsed by, or in any way officially connected with Nvidia, or its subsidiaries, or its affiliates. The official Jetson Nano™ page can be found at https://developer.nvidia.com/embedded/jetson-nano-developer-kit.

The names Nvidia, Jetson Nano, and Orin Nano, as well as related names, marks, emblems, and images belong to their respective owners.
