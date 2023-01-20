# Serenity Rust Bindings

Experimental Rust bindings for Serenity C++ projects.

To build, first clone SerenityOS to some path and export the ``serenity`` directory as SERENITY_SOURCE_DIR.

```
git clone git@github.com:SerenityOS/serenity.git
export SERENITY_SOURCE_DIR=$PWD/serenity
```

Next, have fun with cargo:

```
cargo build
cargo run test-app
```

etc.
