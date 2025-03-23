# 🥡 Introduction 


<p align="center">
  <img src="./caligula-logo.svg"/> <br/>
</p>


Caligula is long project to research and create automatic tofu coagulant machine.

### 🎶 Useful Notes

- run `get_esprs` to add needed compiler to system PATH (`alias get_esprs='export-esp.sh'`)
- to build release, flash, and monitor run `cargo run --release`, it will run runner `espflash  flash --monitor -L defmt` from Config.toml inside `.cargo` folder
- to run monitor only (no build) run `espflash monitor -L defmt --elf target/xtensa-esp32-none-elf/release/caligula`, you need add -L for logging format, and add --elf for defmt know about program.

### 🧑‍🍳 Recipes

- esp-generate -> to scafold esp32 project [repo](https://github.com/esp-rs/esp-generate/)
- espup -> setup rust esp32 toolchain [repo](https://github.com/esp-rs/espup)
- defmt -> test like std rust in no_std [repo](https://github.com/knurling-rs/defmt)
- espflash -> to flashing firmware 
- probe-rs -> to debugging also able to flash

### 🌴 Project Log

- 23 march 2025 20:02, basic esp rust setup with embassy, esp-println and defmt for logging is work
- ⛑️ TOOD: make project to have test like std rust with defmt, so its easier to create and unit testing in no_std

### 🌼 Reference 

- [esp no_std training](https://github.com/esp-rs/no_std-training)
- [official esp32 rust docs](https://docs.espressif.com/projects/rust/)
- [probe-rs](https://github.com/probe-rs/rusty-probe?tab=readme-ov-file)
- [rust-embedded](https://docs.rust-embedded.org/book/intro/tooling.html)
- [embassy-setup-on-esp32](https://pg3.dev/post/13)
- [embassy-book](https://embassy.dev/book/)
- [esp-build](https://github.com/esp-rs/rust-build)
