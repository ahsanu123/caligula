# Step to setup esp32 dev environment 

- install espup, and esp-generate with cargo `cargo install espup`
- on root folder run `esp-generate --chip esp32 my-project-name`, change `--chip` based on your chip, and `my-project-name` to your project name 
- rust will create new folder based on your project name and setup basic compileable example
- run `espup install` to install esp rust ecosystem (its required to build project)
- run `cargo check` to check build your project 

