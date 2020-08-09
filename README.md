# <p align="center">Tipu</p>
<p align="center">
  <img alt="icon" src="https://i.imgur.com/IhaVgsp.png" width="100" height="100"> 
</p>
<p align="center">🌶Spicing the Terminal Experience for JS Developers with: : <a href="https://github.com/tipulabs/tipu-serverhttps://github.com/aryaminus/nazar-server" target="_blank">Tipu-Server</a> </a></p>

[![Run on Repl.it](https://repl.it/badge/github/tipulabs/tipu)](https://repl.it/github/tipulabs/tipu)

## What we think the problem is
You are someone who builds for for users on the web, but the command line is something which is widely different and yet you have to use it one way or other.
So, what if there was an addon which handles the scripts and lets you collaborate right from the terminal?

## What Tipu will mould into

**tipu** initially will act as a is an addon which when run inside a git based ES6 project will open a window where you have an easy access to the scripts that control your codebase.

 You'll be provided with primarily three windows:

    - The central one being where you actually write Shell/BASH commands
    - The top bar will let you fiddle the switches to run scripts directly
    - The sidebar will have two paradigms, one to have a read only session to 'spy' on what you teammates are doing inside the terminal, the other being IRC to communicate with them.

<p align="center">
  <img alt="icon" src="https://i.imgur.com/cMdiytV.png" width="500" height="350"> 
</p>

## Installation

Make sure you have [Rust](https://www.rust-lang.org/tools/install)

Clone the source locally:
```
$ git clone https://github.com/tipulabs/tipu
$ cd tipu
```

Run `tipu-design`:
```
cargo build
cargo run
```
![img](https://i.imgur.com/mEJqNks.png)

Run `tipu-shell`:
```
git checkout feature/shell
cargo build
cargo run
```
![img](https://i.imgur.com/o3wbOgT.png)

Run `tipu-message`:
```
git checkout feature/tcpstream
cargo build
cargo run
```
![img](https://i.imgur.com/QhzJo4q.png)

## Technologies

1. <a href="https://github.com/fdehau/tui-rs" target="_blank">Tui-RS</a>
1. <a href="https://doc.rust-lang.org/std/net/struct.TcpStream.html" target="_blank">TCP Stream</a>
1. <a href="https://github.com/serde-rs/json" target="_blank">Serde-JSON</a>