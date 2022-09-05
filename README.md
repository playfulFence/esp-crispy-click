# esp-crispy-click :crab:
Example for button initialisation and demonstration of work with them.

## Description 
Here you can find a bare-metal example for newcomers that shows, how to initialise and work with buttons on different Espressif boards : 
 - [ESP32](https://www.espressif.com/en/products/socs/esp32)
 - [ESP32-S2](https://www.espressif.com/en/products/socs/esp32-s2)
 - [ESP32-S3-USB-OTG](https://www.espressif.com/en/products/socs/esp32-s3)
 - [ESP32-C3](https://www.espressif.com/en/products/socs/esp32-c3)

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

<a data-flickr-embed="true" href="https://www.flickr.com/photos/196173186@N08/52335295947/in/dateposted-public/" title="crispy-click-esp32c3"><img src="https://live.staticflickr.com/65535/52335295947_af839d581f_c.jpg" width="650" height="350" alt="crispy-click-esp32c3"></a>

>### [Corresponding Wokwi project](https://wokwi.com/projects/341706650098336338)
<br>

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