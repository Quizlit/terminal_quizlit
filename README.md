# terminal_quizlit

## Dev Setup
### Dependencies
#### Clippy
[Clippy](https://github.com/rust-lang/rust-clippy) is our linter.

##### Install
```
rustup update
rustup component add clippy
```

##### Run
```
cargo clippy
```

### pre-commit
[pre-commit](https://pre-commit.com/) is used to create and run our git hooks.

#### Install
This tool assumes that you have python installed.
```
python -m pip install pre-commit
pre-commit install
```

#### Configuration
The pre-commit configuration file is the `.pre-commit-config.yaml` file

### Running unit tests
```
cargo test
``` 
