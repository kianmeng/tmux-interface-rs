tmux_interface
==============

Ubuntu Trusty 14.04: [![Build Status](https://travis-ci.com/AntonGepting/tmux-interface-rs.svg?branch=dev)](https://travis-ci.com/AntonGepting/tmux-interface-rs)


Tmux Interface is a rust language library for communication with TMUX via CLI.


## Usage

1. Add a dependency in your `Cargo.toml`

    ```
    [dependencies]
    tmux_interface = "*"
    ```

2. Add extern crate and use in your source file

    ```
    extern crate tmux_interface;
    ```

3. Use it's functions
    ```
    let tmux = TmuxInterface::new(None);
    tmux.list_sessions();
    ```


## Misc

Used in mosaic - tmux manager

Tested on: tmux 2.8


## License

t
