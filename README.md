# Chipsy

**Chipsy** is a chip-8 emulator written in rust.

CHIP-8 is an interpreted programming language, developed by Joseph Weisbecker. It was initially used on the COSMAC VIP and Telmac 1800 8-bit microcomputers in the mid-1970s. CHIP-8 programs are run on a CHIP-8 virtual machine. It was made to allow video games to be more easily programmed for these computers, but CHIP 8 is still used today, due to its simplicity, and consequentually on any platform and its teaching of programming Binary numbers.

read detail about chip-8 on [wikipedia](https://en.wikipedia.org/wiki/CHIP-8)

## How to run

You need to install SDL2 libs to be able to run **Chipsy**.

### Install SDL2 on ubuntu
```
sudo apt update
sudo apt install libsdl2-dev
```

follow this [link](https://wiki.libsdl.org/Installation) to install SDL2 on other platform.

### Build **Chipsy**
```
cargo build --release
```
find your build output in `./target/release/chipsy`

### Run **Chipsy**
```
./chipsy [ROM_FILE]
```
ROM_FILE is your rom file name with it's path. e.g:
```
chipsy ./roms/INVADERS
```

## Features

Currently **Chipsy** is still on early development stage. It's only cover original (classic) chip-8 implementation. Hope it's able to cover chip-8 extensions such as SUPER-CHIP 1.1 and the relative newcomer XO-CHIP in the future. Below is **Chipsy** target features:

| Features                          |        Status      |
|-----------------------------------|--------------------|
| Original interpreter              | :heavy_check_mark: |
| Chip-8 extensions                 |         :x:        |
| Game Debugger                     |         :x:        |
| Rom Disassembler                  |         :x:        |


# Screenshot
![chipsy-1](https://user-images.githubusercontent.com/11055157/139460907-964582ab-6344-4222-8088-641fc9fdb235.png)

![chip-2](https://user-images.githubusercontent.com/11055157/139460929-858d9e11-64be-4926-87ac-cedeb2c624a8.png)

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License
[MIT](https://choosealicense.com/licenses/mit/)