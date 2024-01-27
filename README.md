<br />
<div align="center">
  <a href="https://github.com/JanitSri/rust-image-art-terminal">
    <img src="docs/rust-logo2.png" alt="Logo" width="150" height="80">
  </a>

  <h2 align="center">Image Display in Terminal</h2>

  <p align="center">
    Display Images in the Terminal using ANSI escape codes
  </p>
</div>

<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->
**Table of Contents** 

- [Build and Running](#build-and-running)
- [Demo](#demo)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

## Build and Running

```shell
cargo run path_to_image [resize_factor] [save_to_file]
```

- path_to_image
  - desc: path to the image to be displayed
- resize_factor
  - desc: image resize factor  
  - default: 1.0
  - valid arguments: 0.0 - 1.0
- save_to_file
  - desc: save the RGBA value at each pixel location
  - default: false
  - valid arguments: true or false

***Note: command line arguments are sequential - proper order is required***

## Demo

![Example Gif](./docs/demo.gif)
