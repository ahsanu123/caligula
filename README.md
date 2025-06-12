# 🥡 Introduction 


<p align="center">
  <img src="./caligula-logo.svg"/> <br/>
</p>


Caligula is long project to research and create automatic tofu coagulant machine.

### 🪪 IO Notes 

currently for development Caligula is use [Sore](https://github.com/ahsanu123/solder-reflow), on Sore 
there is built in input (tact switch) and 2 led (connected to PNP bjt, one is connected to built in SSR) 

**INPUT**

|Schematic Name | Component Name | Connected PIN |
|---------------|----------------|---------------|
| SW3           | IO0            | IO0           | 
| SW4           | IN2            | IO26          |
| SW5           | IN3            | IO25          |
| SW6           | IN1            | IO27          |
| SW7           | IN4            | IO22          |

**OUTPUT**

|Schematic Name | Component Name | Connected PIN |
|---------------|----------------|---------------|
| SSR1          | SSR1           | IO4           | 
| SSR2          | SSR2           | IO32          |

<details>
  <summary>Expand to see schematic </summary>

  ![image](https://github.com/user-attachments/assets/1164a408-79bc-4757-b13f-f8b0a7181529)

  ![image](https://github.com/user-attachments/assets/280a7ee2-9fb8-4b7c-9d4d-2b3ddccd6f85)
  
  ![image](https://github.com/user-attachments/assets/b4e79147-5fbc-4c1e-b486-f3c391e575ed)
  
</details>

### 🎶 Useful Notes

```shell
 To get started, you need to set up some environment variables by running: '. /home/ah/export-esp.sh'
        This step must be done every time you open a new terminal.
            See other methods for setting the environment in https://esp-rs.github.io/book/installation/riscv-and-xtensa.html#3-set-up-the-environment-variables
```
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
- read multithread chapter in rust book
- learn to make peripheral with hal
- learn about interupt and dma
- learn how embassy and esp-hal is connected
- learn and explore embassy docs for esp32
- read embassy_sync for multithread in embassy
- learn embassy feature and learn embassy async
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
