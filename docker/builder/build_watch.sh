#!/usr/bin/env bash

# Load rust config vars
source "$HOME/.cargo/env"

# Set the CWD to the app folder
cd /app

# Build function
build () {
    cargo +nightly build -Z build-std=core,alloc --target powerpc-unknown-eabi.json
    cp /build/target/powerpc-unknown-eabi/debug/rust-wii.elf /build/bin/boot.elf
    echo -e "\e[1;32m Binary build completed. \e[0m"
}

# Run script
echo -e "\e[1;34m Starting initial build... \e[0m"
build
echo -e "\e[1;34m Watch started. \e[0m"
inotifywait -mq -r -e create -e modify -e delete -e move ./src ./data ./Cargo.toml ./build.rs ./wrapper.h ./powerpc-unknown-eabi.json |
    while read dir action file; do
        echo -e "\e[1;34m The file '$file' appeared in directory '$dir' via '$action', rebuilding... \e[0m"
        build
    done