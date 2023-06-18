# vkguide-rs

This is a work-in-progress Rust port of the [Vulkan Guide](https://vkguide.dev/). There are several changes made, including switching from SDL for window creation to GLFW, and trying to use more idiomatic Rust code as opposed to C++11-like code, but the end result should be very similar.

## Organization

This repository uses git branches for chapters. Each of the six chapters in the book (will) have its own branch, named `chapter-0` through `chapter-5`. If you're intending on following along, you can use `chapter-0` as a starting point.

## Goals

While as of right now, there isn't an accompanying book, the intent is to write one specific to this Rust book with [mdBook](https://rust-lang.github.io/mdBook/) once the port is complete.

## License

The original work is licensed under [MIT](https://github.com/vblanco20-1/vulkan-guide/blob/master/LICENSE.txt), and this repository follows the same.

## Contributing

If you'd like to contribute, that'd be greatly appreciated! Please include which branch(es) your patch touches if it's non-obvious. Unless otherwise stated, all contributed code will be assumed to be licensed under the [MIT](https://github.com/niliumgames/vkguide-rs/blob/chapter-0/LICENSE.txt) license. It's preferred if commits are signed; instructions for setting up Github, git, etc can be found [here](https://docs.github.com/en/authentication/managing-commit-signature-verification/signing-commits).
