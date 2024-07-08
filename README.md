# [RustyNES](https://vedant416.github.io/RustyNES/)

RustyNES is a Nintendo Entertainment System (NES) emulator written in Rust.

Experience classic NES games like Super Mario Bros on your computer (linux, windows) or web browser.

Try it online in your browser: [RustyNES](https://vedant416.github.io/RustyNES/)

## Screenshots

### Super Mario Bros.

<img src="screenshots/image1.png" width=250> <img src="screenshots/image2.png" width=250>

## Project Structure

- `RustyNES_core`: A library crate containing the core emulator logic.
- `RustyNES_wasm`: A library crate that compiles `RustyNES_core` to WebAssembly for browser.
- `RustyNES_cli`: A binary crate using `RustyNES_core` to render the emulator output.
- `RustyNES_web_ui`: A web interface using `RustyNES_wasm` to enable running the emulator directly within a browser environment.

## Building

`RustyNES_core` library crate has zero dependencies, but `RustyNES_cli` uses [SDL2](https://www.libsdl.org/) for rendering and input handling.
Ensure that SDL2 is installed on your system for building the `RustyNES_cli`.

### Building for Linux

```bash
cargo build --release
```

`rusty_nes_cli` executable will be in `target/release` directory.

Pass path to the ROM file as an argument to the executable.

```bash
./rusty_nes_cli <path_to_rom_file>
```

### Cross-compiling to Windows

#### Add the Rust target for Windows

```bash
rustup target add x86_64-pc-windows-gnu
```

#### Install mingw-w64

```bash
sudo apt install mingw-w64
```

#### Build the project to Windows

```bash
cargo build --release --target=x86_64-pc-windows-gnu
```

After building `rusty_nes_cli.exe` executable will be in `target/x86_64-pc-windows-gnu/release` directory
and `SDL2.dll` file will be in project root directory.

For running the emulator on Windows, `SDL2.dll` file is required in the same directory as the executable.

Pass path to the ROM file as an argument to the executable.

```bash
rusty_nes_cli.exe <path_to_rom_file>
```

### Building for Web

Install `wasm-pack`:

```bash
cargo install wasm-pack
```

In `RustyNES_web_ui` directory run:

```bash
npm install
npm run dev
```

This will build the RustyNES_wasm lib which will output the WebAssembly file and JavaScript bindings to the `RustyNES_web_ui/public/pkg` directory.

## Keyboard controls

### NES Controller keybindings

| Key                    | NES Button |
| ---------------------- | ---------- |
| <kbd>W</kbd>           | Up         |
| <kbd>A</kbd>           | Left       |
| <kbd>S</kbd>           | Down       |
| <kbd>D</kbd>           | Right      |
| <kbd>L</kbd>           | A          |
| <kbd>K</kbd>           | B          |
| <kbd>Enter</kbd>       | Start      |
| <kbd>Right Shift</kbd> | Select     |

### Extra Features keybindings (Not in the original NES)

For now saving state feature is only available on the `cli` version of the emulator.

| Key          | Action              |
| ------------ | ------------------- |
| <kbd>N</kbd> | Save emulator state |
| <kbd>M</kbd> | Load emulator state |

## Mappers

Mappers implemented:

- Mapper 0 (NROM)
- Mapper 2 (UNROM)
- Mapper4 (MMC3)

## Resources

### Documentation

- [NES Documentation (PDF)](http://nesdev.com/NESDoc.pdf)
- [Nesdev Wiki](https://www.nesdev.org/wiki/Nesdev_Wiki)
- [6502 CPU Reference 1](https://www.c64os.com/post/?p=39)
- [6502 CPU Reference 2](https://www.masswerk.at/6502/6502_instruction_set.html)
- [NES Rendering Overview](https://austinmorlan.com/posts/nes_rendering_overview/)

### Videos

- [The NES Explained] YouTube playlist by [NesHacker]
- [NES Emulator from Scratch] YouTube playlist by [OneLoneCoder]

[NES Emulator from Scratch]: https://www.youtube.com/playlist?list=PLrOv9FMX8xJHqMvSGB_9G9nZZ_4IgteYf
[The NES Explained]: https://youtube.com/playlist?list=PLgvDB6LWam2VDGPgUAMTEEMk0PUtCJs-n&si=Qoquh5uNFiug1iWz
[OneLoneCoder]: https://www.youtube.com/@javidx9
[NesHacker]: https://www.youtube.com/@NesHacker

### Code

- [github.com//zeta0134/rusticnes-sdl](https://github.com/zeta0134/rusticnes-sdl)
- [github.com/OneLoneCoder/olcNES](https://github.com/OneLoneCoder/olcNES)
- [github.com/marethyu/nesty](https://github.com/marethyu/nesty)
- [github.com/takahirox/nes-rust](https://github.com/takahirox/nes-rust)
- [github.com/nathsou/nessy](https://github.com/nathsou/nessy)
- [github.com/fogleman/nes](https://github.com/fogleman/nes)
- [github.com/maxpoletaev/dendy](https://github.com/maxpoletaev/dendy/)
- [github.com/scottferg/Fergulator](https://github.com/scottferg/Fergulator/)
- [github.com/ltriant/nes](https://github.com/ltriant/nes)
- [github.com/spieglt/nestur](https://github.com/spieglt/nestur)
