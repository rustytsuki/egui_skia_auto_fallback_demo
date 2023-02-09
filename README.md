This a skia backend demo, both support opengl and software render mode. As in some cases the hardware accelerations are not available, it will **auto fallback to cpu mode** if the opengl initialized failed.

The main codes are in [https://github.com/rustytsuki/egui/tree/rust-office/crates/eframe/src/native/run](https://github.com/rustytsuki/egui/tree/rust-office/crates/eframe/src/native/run)

some code from [https://github.com/lucasmerlin/egui_skia](https://github.com/lucasmerlin/egui_skia), thanks [lucasmerlin](https://github.com/lucasmerlin)! 

I have fixed some issues,

1. support high dpi in skia software mode
2. corrected skia image color type sent to winit framebuffer
3. avoid copying unnecessary skia surface pixels in software mode to improve render performance

tested both in Mac and Windows, just cargo run, having fun!

note: in windows, you have to upgrade your msvc toolchain up to latest version, included the newest patch, or the skia-safe lib will not be compiled successfully.

just cargo run will auto detect hardware acceleration and will fallback to software mode

`cargo run`

if you want run it in pure cpu software mode

`cargo run --features=skia_force_cpu`

the binary build on windows can be found here

[https://github.com/rustytsuki/egui_skia_auto_fallback_demo/releases](https://github.com/rustytsuki/egui_skia_auto_fallback_demo/releases)