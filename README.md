
## Developing

```
cargo run -- run
```

## Docker

```
docker build -t erichschroeter/fixme:0.1.0 .
docker run --name fixme -p 8080:8080 --rm 32e49dc4576f /usr/local/bin/fixme -v trace run -p 8080 --address 0.0.0.0
#-v $(pwd):/code
```