<div align="center">

<img src="src/asset/logo.png" width="192" height="240">

**A 3D/2D game development framework for Lua, written in Rust.**

</div>

## Usage
Download the latest release from [here](https://github.com/sockentrocken/quiver/releases) and launch Quiver. If no `info_quiver.json`, `main.lua` or folder with the name of `main` is found, Quiver will automatically launch the quick start menu to create a new project. From there, simply create a new project and open the newly made `main.lua` file for further information.

## Documentation
You can find the Lua API and general purpose Quiver documentation [here](https://github.com/sockentrocken/quiver/wiki). Creating a new Quiver project will automatically write a `meta.lua` file for use with the [LuaLS](https://github.com/LuaLS/lua-language-server) LSP.

## Build
Run `cargo build --release` in the root of the Quiver folder.

## License
Quiver has a BSD-2-Clause-Patent license.
