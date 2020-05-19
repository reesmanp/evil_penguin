# Evil Penguin

This is a game written using Rust with the Amethyst game engine (also written in Rust).

## Summary

Evil Penguin is a simple coin collection game where you are a fish who is trying to collect these coins. However, there is an evil penguin that wants to eat you! This penguin will chase you as you navigate the world in search of these coins. Your goal is to collect all the coins before the penguin eats you.

![](https://gfycat.com/scholarlyartistichyracotherium.gif)

## How to Play

Gameplay controls (as of now) are simple:
- Use the arrow keys to navigate the world
  - A note of caution: the world is on ice so you slide around and it takes time for you to accelerate back to full speed. You cannot change direction on a dime.

## How to Download

Currently the only way to download this game is to clone this repository and build the game

You need to have Rust `>= 1.41.0-nightly`. I have not checked yet whether nightly is necessary.

Once Rust is installed, run `cargo build` to build this game and `cargo run` to run it from the command line.
