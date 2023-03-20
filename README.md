# esp32-rust

  https://user-images.githubusercontent.com/77025415/226487016-6d681429-1612-4e83-b4eb-5ef9313be155.mov

  Developed by: <a href="https://www.github.com/gabrielAlonsoCabral">@GabrielAlonsoCabral</a>  
 <br/>

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
    
