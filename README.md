# Chip 8 Emulator #

A Chip 8 Emulator Written in Rust and SDL2.

## Building and Running ##
The emulator was built on Ubuntu 20.04 LTS, and requires ```libsdl2-dev```.
It also runs on Ubuntu 22.04 LTS.

Clone the repository where you'd like, and in your terminal type

```
cargo run [ch8 file]
```

where ```[ch8 file]``` is replaced with the path of the chip8 rom file.

## Controls ##
```
C8 Keypad     Keyboard
123C            1234
456D            QWER
789E     ->     ASDF
A0BF            ZXCV
```
## Screenshots ##

(Games not created by me, see references.)

![Space_Invaders](/screenshots/space_invaders_gameplay.png)

using
```
COLOUR_ACTIVE = 0xE0D9D9;
COLOUR_INACTIVE =0x390000;
```

![Brick](/screenshots/brick_gameplay.png)

using
```
COLOUR_ACTIVE = 0x729FCF;
COLOUR_INACTIVE = 0x001A21;
```

![Maze](/screenshots/maze_demo.png)

using
```
COLOUR_ACTIVE = 0xC89A30;
COLOUR_INACTIVE = 0x221A0F;
```

## References ##
http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/

[Chip8 ROMs](https://github.com/kripod/chip8-roms)

[rust-gb by min050820](https://github.com/min050820/rust-gb)

[Wikipedia](https://en.wikipedia.org/wiki/CHIP-8)

## License ##
[Licensed](./LICENSE) under the MIT License