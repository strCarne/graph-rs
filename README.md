
# graph-rs

Project implements everthing described in task.pdf

## Run Locally

Clone the repo

```bash
  git clone https://github.com/strCarne/graph-rs
```

Go to the project directory

```bash
  cd graph-rs
```

Then you have 2 options:
1. Run natively on your machine
```bash
  cargo build --release
  
  ./target/release/graph-rs --help
```
2. Run in docker container
```bash
  # 1. Build docker image
  docker build -t graph-rs:latest .

  # 2. Run the container. 
  # But make sure to bind the volume!
  # This will output you help clause.
  docker run --rm -it -v .:/app/data --name graph-rs graph-rs

  # To specify the file name.
  # docker run --rm -it -v .:/app/data --name graph-rs graph-rs ./data/<file_name.tgf>
  #
  # Example: 
  # docker run --rm -it -v .:/app/data --name graph-rs graph-rs ./data/demo.tgf
```


## Running Tests

To run tests, run the following command

```bash
  cargo test
```

## API Reference

1. Build the reference
```bash
  cargo doc
```

2. Open the reference
```bash
  cargo doc --open
```