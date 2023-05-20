# script-runner

A simple CLI script runner that allows you to execute multiple scripts in parallel, using OS threads.

The idea is to have a alias in your shell that allows you to run all scripts in a folder, for example:

```bash
alias work-scripts="script-runner /home/user1/Documents/scripts/"
```

The order of the scripts in the selection menu is defined by the depth of the folders inside the folder passed as argument. The deeper the folder, the higher the priority.

### Known issues

- If you `ctrl-c` during the selection the terminal can get messed up. To fix it just reset the terminal. To cancel the selection just don't select anything and press `enter`.

---

## Usage

```bash
script-runner <path-to-folder-with-scripts>
```

You can select multiple scripts by pressing `space` and then `enter` to run them in parallel.

## Build

```bash
cargo build --release
```

## Install

```bash
cargo install --path .
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
