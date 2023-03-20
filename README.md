# esp32-rust

### Build
- Terminal approach:

    ```
    scripts/build.sh  [debug | release]
    ```
    > If no argument is passed, `release` will be used as default

### Wokwi Simulation
When using a custom Wokwi project, please change the `WOKWI_PROJECT_ID` in
`run-wokwi.sh`. If no project id is specified, a DevKit for esp32 will be
used.
> **Warning**
>
>  ESP32-S3 is not available in Wokwi

- Terminal approach:

    ```
    scripts/run-wokwi.sh [debug | release]
    ```
    > If no argument is passed, `release` will be used as default
    
#### Debuging with Wokwi

Wokwi offers debugging with GDB.

- Terminal approach:
    ```
    $HOME/.espressif/tools/xtensa-esp32-elf/esp-2021r2-patch3-8.4.0/xtensa-esp32-elf/bin/xtensa-esp32-elf-gdb target/xtensa-esp32-espidf/debug/esp32-rust -ex "target remote localhost:9333"
    ```

    > [Wokwi Blog: List of common GDB commands for debugging.](https://blog.wokwi.com/gdb-avr-arduino-cheatsheet/?utm_source=urish&utm_medium=blog)