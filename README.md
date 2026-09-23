# aula-kb-tool

Cross-platform tool for configuring Aula keyboards. This project aims to become a replacement for the proprietary and Windows-only Aula app.

![user interface](https://raw.githubusercontent.com/nebular0/aula-kb-tool/refs/heads/main/assets/screenshot.png)

## List of features (roadmap)
- Static RGB lighting: done ✅
- RGB lighting modes: in development
- RGB dynamic effect: unplanned ❌
- Key-mapping: planned, not in active development
- Keyboard settings (debouncing, sleep time, etc.): planned, not in active development

## Compatibile devices
- Aula F75
- ... virtually any keyboard of the same class

## Known limits & Caveats
Currently you can only configure keyboards via a USB connection.

- On Linux you should have udev permissions to access hidraw devices. https://askubuntu.com/questions/15570/configure-udev-to-change-permissions-on-usb-hid-device
- On Windows you have to select the entry (HID path) containing "Col06".

## Contributing
Finalizing further this project requires owning different Aula keyboards, which is an expensive and tough feat to achieve.

However, **anyone** who owns a keyboard NOT included in the compatibile devices can contribute immensely even without having hands-on experience with device spoofing/reverse engineering.

Feel free to create an inquiry in the [issues](/issues) page.

## AI disclosure
No AI used was for any given part of this project. This is a months-old effort to reverse engineer a given software with no documentation in hand and as such it deserves to be treated with dignity. Attempting to "contribute" with the help of AI will lead to a ban.

For more visit [PauseAI](https://pauseai.info).
