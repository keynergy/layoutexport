# layoutexport
Simple CLI tool that creates Linux XKB files from Keynergy layout files.

## Usage
```sh
layoutexport -l LAYOUT.toml
```
A new file will be created in the current directory. You can then put this layout file into `/usr/share/X11/xkb/symbols/`.
