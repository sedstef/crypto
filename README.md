# Website demonstrating cryptographic algorithm

## Build with podman
```bash
podman build --target prod -t sedstef/crypto .
```

## Run with podman
```bash
podman run --rm -it -p 3000:3000 $(podman build -q --target prod .)
```
