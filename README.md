# fe10msg
Utility for editing fe10 text data.

## Instructions
1. Download the latest release from [https://github.com/thane98/fe10msg/releases](https://github.com/thane98/fe10msg/releases)
2. Open a terminal in the same directory as `fe10msg.exe`
3. Run `.\fe10msg.exe extract {path to your .m file}`. If the file is valid, you will get a `.yml` file with the same name in the directory of the `.m` file.
4. Make your edits to the `.yml` file.
5. Run `.\fe10msg.exe import {path to your .yml file`. If the file is valid, you will get a `.m` file with the same in the directory of the `.m` file. **This will overwrite existing files.**
6. Test in game!
