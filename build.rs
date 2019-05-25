extern crate cc;
use std::path::Path;

fn main() {
    let path: &Path = Path::new("./rpi-rgb-led-matrix/lib");
    cc::Build::new()
        .file(path.join("hardware-mapping.c"))
        .compile("hardware-mapping");

    cc::Build::new()
        .cpp(true)
        .include(path.join("../include"))
        .file(path.join("bdf-font.cc"))
        .file(path.join("content-streamer.cc"))
        .file(path.join("framebuffer.cc"))
        .file(path.join("gpio.cc"))
        .file(path.join("graphics.cc"))
        .file(path.join("led-matrix.cc"))
        .file(path.join("led-matrix-c.cc"))
        .file(path.join("multiplex-mappers.cc"))
        .file(path.join("options-initialize.cc"))
        .file(path.join("pixel-mapper.cc"))
        .file(path.join("thread.cc"))
        .file(path.join("transformer.cc"))
        .compile("rgbmatrix");
}
