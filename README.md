# Tic-Rac-Go

[TUI.webm](https://github.com/ximon-x/Tic-Rac-Go/assets/56292632/61c03bb0-43ba-47be-9c04-5ea785a4c88b)

A Simple TUI application built using Ratatui, Rust and Go.

# Prerequisites
1. The Rust toolchain.
2. Go
   
# Setting up
1. First clone the repo.
2. From the server directory, run `go run cmd/main.go`
3. Navigate into the client directory and run `cargo run`

# How to play
You can select the board grid size at the game's start. The board uses chessboard coordinates: A1 or C6, to identify cells. The TUI application reads from stdin and will only send good moves to the backend. It clears the TUI input when a wrong input is entered. The program exits when the game is over or a win is encountered.
