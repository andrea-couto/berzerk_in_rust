# Berzerk

A berzerk simple remake in Rust

## How to run it

Currently only runs on Windows 10 with Rust 1.3

In the project folder type `cargo run` this will bring up the game.

## About the game

The goal of the game is to eliminate all the enemies and leave the board. Once you pass 4 levels you win the game. 

Each time you get to a new level the number of enemies increase and so does the chance that they will shoot at you. 

The enemies will slowly move toward you, use this to your advantage.

There is a little forgiveness for overlapping with the wall (the player does not loose a life if only a few pixels overlap).

## Game Controls

Use the arrow keys to move the player

Use the spacebar to shoot

## TODO

To save on time I reused the same background for all the levels. The game would be more scalable and interesting if tiles were used instead. 

The game mod could be divided into other modules. For instance the logic that creates the background could be made into its own file. 

Player and Enemy have overlapping functions and properties that could be derived from a base struct.

Tests need improvement and more coverage 

For tests in modules like Player or Enemy I created an object for each test and that is a waste of resources. However, I could not find documentation on how to create that object inside the test module and use it in tests..
