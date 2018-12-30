# deletemojo
Remove the mojo installer from linux games (e.g., those downloaded from gog)! 
deletemojo unpacks a mojoinstaller shell script and creates a file that only contains the game data ("game.tar.gz").
This archive can then be extracted.

Heavily inspired by [gogextract](https://github.com/Yepoleb/gogextract), but written in Rust!

## Limitations
- Doesn't currently support specifiying an output file
- Doesn't currently extract the game data
