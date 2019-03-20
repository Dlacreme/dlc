DLC MOD
=======

Not a Crate nor a Library.
A module you should install manually and edit.

# Install

## Dependancies

You will have to manually add the depancies in your Cargo.toml:
```
[dependencies]
toml = "0.5.0"
rustc-serialize = "0.3"
log = "0.4"
```

## Download

```
$> cargo new toto --bin && cd toto
_
$> cd src && git clone https://github.com/Dlacreme/dlc
$> rm -rf ./dlc/.git* ./dlc/*.md && cd ..
```

# What it is

## Config

Here is the file used in this example. You should update it to match your Config struct
```
[app]
version = 0.1
name = "Sutro"
prod = false

[network]
address = "127.0.0.1:4242"
```

## Toml Parser
