# Gamehacking.Academy but made with Rust
The first few "hacks" made target a game called "The Battle for Wesnoth" mainly version 1.14.9 - so they might not work if you are testing this locally on a different version

Note; the game runs on 32bit so for the "hack" tool to work properly you also should run it on 32bit

## ExternalMemoryHack
this contains one of the first "hacks" made
Using winapi to hook into the process and edit the value of how much gold the player has.

## DLLMemoryHack
(lib.rs)
The .DLL itself, it works if injecting it with Cheat Engine.
You can then pressing 'm' sets your gold to 9999

(main.rs)
The injector, which is a bit of WIP and unusable so far.
Currently it crashes on injecting the dll, i'm unable to see why yet

Ingame screenshot after pressing 'm':
![image](https://user-images.githubusercontent.com/83826811/201967081-46767ac9-047e-471e-8098-c2ab93ccfb69.png)
