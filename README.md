<div align="center">

<img src="src/asset/card.png" width="192" height="240">

**A 3D/2D game development framework for Lua, written in Rust.**

</div>

## Feature List

As part of the Rust standard library:
* JSON, YAML, TOML, XML, INI serialization/deserialization
* Native window dialog
* File system notifier
* 3D/2D physics engine
* .ZIP creation/extraction
* HTTP GET/POST, socket networking
* Discord and Steam integration
* Video playback

As part of the Lua standard library:

* Input abstraction
* Console/logger
* Math library
* Virtual file-system
* Scene manager
* User interface

## Usage
Download the latest release from [here](https://github.com/sockentrocken/quiver/releases) and launch Quiver. If no `info.json`, `main.lua` or folder with the name of `main` is found, Quiver will automatically launch the quick start menu to create a new project. From there, simply create a new project and open the newly made `main.lua` file for further information.

## Example

To run any game example, simply download either example, move the Quiver executable into the folder (like so:)

```
example_2D/
	quiver
	main.lua
	data/
``` 

and launch Quiver.

## Documentation

You can find the Rust API and general purpose Quiver documentation [here](https://github.com/sockentrocken/quiver/wiki). Creating a new Quiver project will automatically write a `meta.lua` file for use with the [LuaLS](https://github.com/LuaLS/lua-language-server) LSP.

## Build
Run `cargo build --release` in the root of the Quiver folder.

## Acknowledgement
[raylib](https://github.com/raysan5/raylib) - main back-end for Quiver's 3D/2D functionality.

[mlua](https://github.com/mlua-rs/mlua) - Lua library for abstracting much of Lua's C API for Rust.

[Rapier](https://github.com/dimforge/rapier) - 3D/2D physics engine.

## License
Quiver has a BSD-2-Clause-Patent license.
