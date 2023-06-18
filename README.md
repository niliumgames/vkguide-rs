# vkguide-rs

This is a work-in-progress Rust port of the [Vulkan Guide](https://vkguide.dev/). There are several changes made, including switching from SDL for window creation to GLFW, and trying to use more idiomatic Rust code as opposed to C++11-like code, but the end result should be very similar.

## Organization

This repository uses git branches for chapters. Each of the six chapters in the book (will) have its own branch, named `chapter-0` through `chapter-5`. If you're intending on following along, you can use `chapter-0` as a starting point.

## Goals

While as of right now, there isn't an accompanying book, the intent is to write one specific to this Rust book with [mdBook](https://rust-lang.github.io/mdBook/) once the port is complete.

## License

The original work is licensed under [MIT](https://github.com/vblanco20-1/vulkan-guide/blob/master/LICENSE.txt), and this repository follows the same.

## Contributing

See [CONTRIBUTING.md](https://github.com/niliumgames/vkguide-rs/blob/chapter-0/CONTRIBUTING.md)
