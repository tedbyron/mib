# mibi

windows (admin): connect USB device to WSL

```shell
winget install --interactive --exact dorssel.usbipd-win
# VID:PID is 0d28:0204 (NXP:ARM mbed)
usbipd wsl list
usbipd wsl attach --busid <BUSID>
# lsusb (usbutils) in wsl shows an NXP ARM mbed device
```
