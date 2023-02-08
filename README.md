This a skia backend demo, both support opengl and software render mode. As in some cases the hardware accelerations are not available, it will **auto fallback to cpu mode** if the opengl initialized failed.

The main codes are in [https://github.com/rustytsuki/egui/tree/rust-office](https://github.com/rustytsuki/egui/tree/rust-office)

some code from [https://github.com/lucasmerlin/egui_skia](https://github.com/lucasmerlin/egui_skia), thanks [lucasmerlin](https://github.com/lucasmerlin)!

tested both in Mac and Windows, just cargo run, having fun!

note: in windows, you have to upgrade your msvc toolchain up to latest version, included the newest patch, or the skia-safe lib will not be compiled successfully.

