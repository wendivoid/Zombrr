# Zombrr

  Entirely modular FPS written in bevy. My goal is to hopefully have a fully
Customizable FPS shooter game in the vain of Call of Duty Zombies, but with easily
created and customizable maps, characters, weapons, and game modes.

**WARN**: I fully expect this project to go no where and never amount to anything.

![alt text](./resources/github/Sandbox.png)

## crates

#### bevy_devtools

This is a simple crate I've been working on to add a customizable 'DevTools' window
That comes packed with a bunch of useful tools like dumping the render_graph to a dot file with bevy_mod_debugdump or saving the world as a scene file. I was going to eventually publish this crate, but after i've used if for a few months now i'm still not satisfied with it, for example requires more unsafe code than id like.
Development has moved to [bevy_devtools](https://github.com/Bytebuddha/bevy_devtools)

#### bevy_hilt

![alt text](./resources/github/hilt.png)

This is a debug renderer for rapier3d. In a perfect world i could upstream this to
bevy_rapier in the next couple months and not maintain this here. My development on this has moved to [bevy_hilt](https://github.com/Bytebuddha/bevy_hilt).

#### bevy_sky

Simple sky implementation refactored version of the library used in [bevy_minecraft_clone](https://github.com/hypercubed-music/bevy_minecraft_clone)
I was going to eventually send a PR to port some features back upstream at some point
but I'm not really satisfied / it kinda sucks also i think a broke something.

### source

#### core

Contains all the non bevy needing things, like the MapMeta file descriptions. and the game resource object's/state.

#### gltf

Currently this library is just a shim that loads the gltf extras attribute into a custom component.

#### zombrr

All of the game related stuff, eventually ill pull the larger parts out into there own libraries.

## Running

**NOTE**: git-lfs is required to download the assets

**NOTE**: I only have a linux laptop so i can only test on Linux.

I'm using cargo aliases to build this project take a look at the bottom of the .cargo/config.toml file to get a list of all the available run commands, but here are the most useful three

```sh
cargo run:hilt:drop # run the bevy_hilt drop example
```

```sh
cargo run:hilt:positioned # run the bevy_hilt positioned example
```

```sh
cargo run:sandbox # Run the sandbox just opens the world and loads a map currently
```

### Thanks

I've only been able to create this by reading literally 100's or so repositories from around the web but i'll just list a few I've stumbled upon and used for inspiration.

-   [bevy_gizmos](https://github.com/lassade/bevy_gizmos)
-   [bevy_prototy_character_controller](https://github.com/superdump/bevy_prototype_character_controller)
-   [bevy_polyline](https://github.com/ForesightMiningSoftwareCorporation/bevy_polyline)
-   [bevy_transform_gizmo](https://github.com/ForesightMiningSoftwareCorporation/bevy_transform_gizmo)
