# dungeon-project-management

A dungeon delver and typing tutor built using rust and the macroquad crate.

# Compilation
This game can be run by the following command `cargo run -r` or `cargo run --release` (from the dungeon directory). While this game can be run in debug mode it will take quite a while to load initially.

## Rules/Description

The aim of the game is is to get to the final level without dying. You can move around the map by clicking on a node and you will be moved there walking onto a chest activates it and walking onto an enemy (represented by the go-gopher) initates combat
### Combat
In combat the player will take damage every few seconds and the goal is to type the provided sentence(s) 100% correctly in the shortest time possible without taking damage.
### Victory
The game is won when the player lands on the final 'crown'.
