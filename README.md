
# Build with podman
```bash
podman build .
```

# Run with podman
```bash
podman run --rm -it -p 3000:3000 $(podman build -q .)
```
