# Examples for ESP32-S3

Embassy async examples for the ESP32-S3, targeting the Heltec WiFi LoRa 32 V3 board. These demonstrate using Embassy's executor and timer with the [esp-hal](https://github.com/esp-rs/esp-hal) ecosystem on an Xtensa target.

## Directory Structure

```
examples/esp32-s3/
├── .cargo/
│   └── config.toml     # Xtensa target, espflash runner, build-std
├── src/
│   └── bin/
│       └── blinky.rs    # Async blinky using GPIO35 (active high)
├── Cargo.toml           # Dependencies from crates.io
└── build.rs             # Linker script + app descriptor for espflash v4+
```

## Key Differences from STM32 Examples

- **Dependencies** come from crates.io rather than local paths, because the ESP HAL (esp-hal) is maintained externally by the [esp-rs/esp-hal](https://github.com/esp-rs/esp-hal) project, not in this Embassy repo. The Embassy crate versions (executor 0.7.0, time 0.4.0) are pinned to match what esp-hal-embassy expects.
- **Target:** `xtensa-esp32s3-none-elf` instead of ARM Cortex-M targets. This requires `build-std = ["core", "alloc"]` since Xtensa doesn't have a pre-built standard library.
- **Toolchain:** Requires the ESP Rust toolchain (install via [espup](https://github.com/esp-rs/espup)).
- **Flashing:** Uses `espflash flash --monitor` instead of probe-rs.
- **Logging:** Uses `esp-println` + `log` crate instead of `defmt` + `defmt-rtt`, as is conventional for ESP targets.
- **Entry point:** Uses `#[esp_hal_embassy::main]` macro instead of `#[embassy_executor::main]`.

## Setup

Install the ESP Rust toolchain (one-time):
```
cargo install espup
espup install
```

Before each terminal session, source the environment:
```
. $HOME/export-esp.sh
```

## Build

To build an example:
```
RUSTUP_TOOLCHAIN=esp cargo build --release --bin blinky
```

To build and flash (with the board connected via USB):
```
RUSTUP_TOOLCHAIN=esp cargo run --release --bin blinky
```

## Adapting to Other ESP32-S3 Boards

The blinky example uses GPIO35 for the LED (Heltec WiFi LoRa 32 V3). If you're using a different board, update the GPIO pin number in `src/bin/blinky.rs` to match your board's LED pin.

