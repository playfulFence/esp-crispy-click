# esp-crispy-click :crab:
Example for button initialisation and demonstration of work with them.

## Description 
Here you can find a simple bare-metal example for newcomers that shows, how to initialise and work with buttons on different Espressif boards: 
 - [ESP32](https://www.espressif.com/en/products/socs/esp32)
 - [ESP32-S2](https://www.espressif.com/en/products/socs/esp32-s2)
 - [ESP32-S3-USB-OTG](https://www.espressif.com/en/products/socs/esp32-s3)
 - [ESP32-C3](https://www.espressif.com/en/products/socs/esp32-c3)

 > **Warning**
>
>  ESP32-S3 is not available in Wokwi

 >### **Important** : below in this branch you can find pin connection for REAL hardware, not for Wokwi. Pin connection for every chip for Wokwi can be found in corresponding branches

## Some screenshots

### ESP32 with ILI9341 display and two buttons

<a data-flickr-embed="true" href="https://www.flickr.com/photos/196173186@N08/52335293267/in/dateposted-public/" title="crispy-click-esp32"><img src="https://live.staticflickr.com/65535/52335293267_ca257813bc_c.jpg" width="570" height="500" alt="crispy-click-esp32"></a>

>### [Corresponding Wokwi project](https://wokwi.com/projects/341705886867128915)
<br>

### ESP32-S2 with ILI9341 display and two buttons

<a data-flickr-embed="true" href="https://www.flickr.com/photos/196173186@N08/52336243416/in/dateposted-public/" title="crispy-click-esp32s2"><img src="https://live.staticflickr.com/65535/52336243416_e7f7d2894f_c.jpg" width="470" height="570" alt="crispy-click-esp32s2"></a>

>### [Corresponding Wokwi project](https://wokwi.com/projects/341676100758274644)
<br>

### ESP32-C3 with ILI9341 display and two buttons

<a data-flickr-embed="true" href="https://www.flickr.com/photos/196173186@N08/52336708125/in/dateposted/" title="crispy-click-esp32c3"><img src="https://live.staticflickr.com/65535/52336708125_24c263a48e_c.jpg" width="650" height="350" alt="crispy-click-esp32c3"></a>

>### [Corresponding Wokwi project](https://wokwi.com/projects/341706650098336338)
<br>

 >### **Important** : If you want to set your own pins for buttons or change the number of buttons, you should customize it in [these](https://github.com/playfulFence/esp-crispy-click/blob/main/src/main.rs#L287-L289) lines of code

## Build and flash

### Build for ESP32-S3-USB-OTG

```
cargo espflash --release --target xtensa-esp32s3-none-elf 
```

### Build for ESP32

```
cargo espflash --release --target xtensa-esp32-none-elf 
```

### Build for ESP32-S2

```
cargo espflash --release --target xtensa-esp32s2-none-elf 
```

### Build for ESP32-C3

```
cargo espflash --release --target riscv32imac-unknown-none-elf
```

## Dev Containers
This repository offers Dev Containers supports for:
-  [Gitpod](https://gitpod.io/)
    - ESP32
        - [![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/playfulFence/esp-crispy-click/tree/target/esp32)
    - ESP32-S2
        - [![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/playfulFence/esp-crispy-click/tree/target/esp32s2)
    - ESP32-C3
        - [![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/playfulFence/esp-crispy-click/tree/target/esp32c3)

-  [VS Code Dev Containers](https://code.visualstudio.com/docs/remote/containers#_quick-start-open-an-existing-folder-in-a-container)
-  [GitHub Codespaces](https://docs.github.com/en/codespaces/developing-in-codespaces/creating-a-codespace)
> **Note**
>
> In order to use Gitpod the project needs to be published in a GitLab, GitHub,
> or Bitbucket repository.
>
> In [order to use GitHub Codespaces](https://github.com/features/codespaces#faq)
> the project needs to be published in a GitHub repository and the user needs
> to be part of the Codespaces beta or have the project under an organization.

If using VS Code or GitHub Codespaces, you can pull the image instead of building it
from the Dockerfile by selecting the `image` property instead of `build` in
`.devcontainer/devcontainer.json`. Further customization of the Dev Container can
be achived, see [.devcontainer.json reference](https://code.visualstudio.com/docs/remote/devcontainerjson-reference).

When using Dev Containers, some tooling to facilitate building, flashing and
simulating in Wokwi is also added.
### Build
- Terminal approach:

    ```
    scripts/build.sh  [debug | release]
    ```
    > If no argument is passed, `release` will be used as default


-  UI approach:

    The default build task is already set to build the project, and it can be used
    in VS Code and Gitpod:
    - From the [Command Palette](https://code.visualstudio.com/docs/getstarted/userinterface#_command-palette) (`Ctrl-Shift-P` or `Cmd-Shift-P`) run the `Tasks: Run Build Task` command.
    - `Terminal`-> `Run Build Task` in the menu.
    - With `Ctrl-Shift-B` or `Cmd-Shift-B`.
    - From the [Command Palette](https://code.visualstudio.com/docs/getstarted/userinterface#_command-palette) (`Ctrl-Shift-P` or `Cmd-Shift-P`) run the `Tasks: Run Task` command and
    select `Build`.
    - From UI: Press `Build` on the left side of the Status Bar.

### Flash

> **Note**
>
> When using GitHub Codespaces, we need to make the ports
> public, [see instructions](https://docs.github.com/en/codespaces/developing-in-codespaces/forwarding-ports-in-your-codespace#sharing-a-port).

- Terminal approach:
  - Using `flash.sh` script:

    ```
    scripts/flash.sh [debug | release]
    ```
    > If no argument is passed, `release` will be used as default

- UI approach:
    - From the [Command Palette](https://code.visualstudio.com/docs/getstarted/userinterface#_command-palette) (`Ctrl-Shift-P` or `Cmd-Shift-P`) run the `Tasks: Run Task` command and
    select `Build & Flash`.
    - From UI: Press `Build & Flash` on the left side of the Status Bar.
- Any alternative flashing method from host machine.


### Wokwi Simulation
When using a custom Wokwi project, please change the `WOKWI_PROJECT_ID` in
`run-wokwi.sh`. If no project id is specified, a DevKit for esp32c3 will be
used.
> **Warning**
>
>  ESP32-S3 is not available in Wokwi

- Terminal approach:

    ```
    scripts/run-wokwi.sh [debug | release]
    ```
    > If no argument is passed, `release` will be used as default

- UI approach:

    The default test task is already set to build the project, and it can be used
    in VS Code and Gitpod:
    - From the [Command Palette](https://code.visualstudio.com/docs/getstarted/userinterface#_command-palette) (`Ctrl-Shift-P` or `Cmd-Shift-P`) run the `Tasks: Run Test Task` command
    - With `Ctrl-Shift-,` or `Cmd-Shift-,`
        > **Note**
        >
        > This Shortcut is not available in Gitpod by default.
    - From the [Command Palette](https://code.visualstudio.com/docs/getstarted/userinterface#_command-palette) (`Ctrl-Shift-P` or `Cmd-Shift-P`) run the `Tasks: Run Task` command and
    select `Build & Run Wokwi`.
    - From UI: Press `Build & Run Wokwi` on the left side of the Status Bar.

> **Warning**
>
>  The simulation will pause if the browser tab is in the background.This may
> affect the execution, specially when debuging.

#### Debuging with Wokwi

Wokwi offers debugging with GDB.

- Terminal approach:
    ```
    $HOME/.espressif/tools/riscv32-esp-elf/esp-2021r2-patch3-8.4.0/riscv32-esp-elf/bin/riscv32-esp-elf-gdb target/riscv32imc-esp-espidf/debug/esp_clock -ex "target remote localhost:9333"
    ```

    > [Wokwi Blog: List of common GDB commands for debugging.](https://blog.wokwi.com/gdb-avr-arduino-cheatsheet/?utm_source=urish&utm_medium=blog)
- UI approach:
    1. Run the Wokwi Simulation in `debug` profile
    2. Go to `Run and Debug` section of the IDE (`Ctrl-Shift-D or Cmd-Shift-D`)
    3. Start Debugging by pressing the Play Button or pressing `F5`
    4. Choose the proper user:
        - `esp` when using VS Code or GitHub Codespaces
        - `gitpod` when using Gitpod
        