# 🥡 Introduction 

<p align="center">
  <img src="./caligula-logo.svg"/> <br/>
</p>

--- 
 <img src="./hardware/docs/caligula.svg"/> <br/>


### 🎶 Useful Notes

```shell
To get started, you need to set up some environment variables by running: '. /home/ah/export-esp.sh'
This step must be done every time you open a new terminal.
See other methods for setting the environment in https://esp-rs.github.io/book/installation/riscv-and-xtensa.html#3-set-up-the-environment-variables
```

- to use usb from windows inside wsl you need bridge usb device from windows to wsl with `usbipd`

  ```shell
  C:\Windows\System32> usbipd list
  Connected:
  BUSID  VID:PID    DEVICE                                                        STATE
  1-1    feed:0ffa  USB Input Device                                              Not shared
  1-2    0000:3825  USB Input Device                                              Not shared
  1-4    0bda:0129  Realtek USB 2.0 Card Reader                                   Not shared
  1-5    13d3:56cb  USB2.0 HD IR UVC WebCam                                       Not shared
  1-6    1366:0101  J-Link driver                                                 Not shared
  1-10   8087:0026  Intel(R) Wireless Bluetooth(R)                                Not shared

  Persisted:
  GUID                                  DEVICE


  C:\Windows\System32> usbipd bind --busid 1-6

  C:\Windows\System32> usbipd attach --wsl --busid 1-6
  usbipd: info: Using WSL distribution 'Arch' to attach; the device will be available in all WSL 2 distributions.
  usbipd: info: Detected networking mode 'nat'.
  usbipd: info: Using IP address 172.27.144.1 to reach the host.

  C:\Windows\System32>
  ```


  then you can access the usb device as in linux


- run `get_esprs` to add needed compiler to system PATH (`alias get_esprs='export-esp.sh'`)
- to build release, flash, and monitor run `cargo run --release`, it will run runner `espflash  flash --monitor -L defmt` from Config.toml inside `.cargo` folder
- to run monitor only (no build) run `espflash monitor -L defmt --elf target/xtensa-esp32-none-elf/release/caligula`, you need add -L for logging format, and add --elf for defmt know about program.
- run `espflash board-info` to get connected board information
- to flash and run, `cargo run --release --bin rust_file_name.rs`
- embassy-sync blog -> https://blog.theembeddedrustacean.com/sharing-data-among-tasks-in-rust-embassy-synchronization-primitives

### 🧑‍🍳 Recipes

- esp-generate -> to scafold esp32 project [repo](https://github.com/esp-rs/esp-generate/)
- espup -> setup rust esp32 toolchain [repo](https://github.com/esp-rs/espup)
- defmt -> test like std rust in no_std [repo](https://github.com/knurling-rs/defmt)
- espflash -> to flashing firmware 
- probe-rs -> to debugging also able to flash

### 🪗 TODO Log

learn rust embassy ecosistem with esp-rs 

- ~~map all predefined SORE button and led~~
- ~~read multithread chapter in rust book~~
- ~~learn to make peripheral with hal~~
- learn about interupt and ~~dma~~
- ~~learn how embassy and esp-hal is connected~~
- ~~learn and explore embassy docs for esp32~~
- ~~read embassy_sync for multithread in embassy~~
- ~~learn embassy feature and learn embassy async~~
- search for match sensor and actuator
- make TMC2209 Stepper motor driver for actuator

### 🌴 Project Log

- 23 march 2025 20:02, basic esp rust setup with embassy, esp-println and defmt for logging is work
- ⛑️ TOOD: make project to have test like std rust with defmt, so its easier to create and unit testing in no_std
- 10 Mei 2025 13:06, done skiming https://docs.esp-rs.org/book

### 🌼 Reference 

- [esp no_std training](https://github.com/esp-rs/no_std-training)
- [official esp32 rust docs](https://docs.espressif.com/projects/rust/)
- [probe-rs](https://github.com/probe-rs/rusty-probe?tab=readme-ov-file)
- [rust-embedded](https://docs.rust-embedded.org/book/intro/tooling.html)
- [embassy-setup-on-esp32](https://pg3.dev/post/13)
- [embassy-book](https://embassy.dev/book/)
- [esp-build](https://github.com/esp-rs/rust-build)
