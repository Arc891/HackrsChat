# HackrsChat
```
$ cargo run -p hackrschat
>   __  __           __             ________          __ 
   / / / /___ ______/ /____________/ ____/ /_  ____ _/ /_
  / /_/ / __ `/ ___/ //_/ ___/ ___/ /   / __ \/ __ `/ __/
 / __  / /_/ / /__/ ,< / /  (__  ) /___/ / / / /_/ / /_  
/_/ /_/\__,_/\___/_/|_/_/  /____/\____/_/ /_/\__,_/\__/  
```

HackrsChat is a personal-use self-host TUI chat application for you and your friends/community, written in Rust, making use of the Cursive library.
This chat application will have a privacy-focused approach, with the ability to self-host the server and client, and with good encryption and safety practices, to be sure that your data is safe and secure.
This project is currently a work in progress, feel free to contribute!


## Development

After cloning this repository and with the [Devcontainers VSCode](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers) extension you can open the command runner to launch the following command: _"DevContainer: Rebuild and Reopen in Container"_. This will open a VSCode server inside a docker container with all necessary dependencies installed (see [`.devcontainer/Dockerfile`](.devcontainer/Dockerfile)).

We can specify the command targets in the `Justfile` (a Makefile alternative). Which means we can run the application with:

``` bash
just run
```


## Demo

https://github.com/user-attachments/assets/875b51e0-e4cb-4c58-8fda-016139f2115f

## Roadmap

- [ ] Basic TUI layout
  - [x] Welcome screen with logo
  - [x] Login screen
  - [x] Registration screen
  - [x] Main menu
    - [x] Chat list
    - [x] Terminal/command box
  - [ ] Chat layout
    - [ ] Messages position & style
    - [x] Chat input
    - [x] Chat commands (currently use terminal commands)
- [ ] Actual server integration
  - [x] Echo server
  - [ ] Basic chat functionality
  - [ ] User authentication
  - [ ] User registration
  - [ ] Basic chat commands
  - [ ] Usable API for external applications
- [ ] More to come...
