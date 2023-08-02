
## Developing

```
cargo run -- run
```

## Docker (manual)

```bash
docker build -t erichschroeter/fixme:0.1.0 .
docker run --name fixme -p 8080:8080 --rm template-rust-webapp-app /usr/local/bin/fixme -v debug run -p 8080 --address 0.0.0.0
#-v $(pwd):/code
```

## Docker compose

```bash
docker compose -d --build
```
