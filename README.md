# GameTorch

<img src="/mule.png" alt="Mule left walk" style="max-width:150px; max-height:150px;">

walking to the left

<img src="/mule_correct.webp" alt="Mule left walk final" style="max-width:150px; max-height:150px;">

you can

- ✨ Create, edit, and [animate](https://gametorch.app/sprite-animator) with English.
- ⬆️ Publish to [Creative Commons](https://gametorch.app/commons) with one click.
- 💰 Get rewarded if your assets get featured.
- 🎨 Free, no sign up required [image to pixel art tool](https://gametorch.app/image-to-pixel-art)!

---

# GameTorch CLI

Welcome to the **GameTorch** command-line interface and library. 🕹️⚡️

---

## Prerequisites

1. **Rust toolchain** – Install Rust using the official installer:
   <https://www.rust-lang.org/tools/install>

   After installation, ensure `cargo --version` works in your terminal.

---

## Building

Clone this repository and run:

```bash
cargo build --release
```

The optimized binary will be produced at:

```
target/release/gametorch
```

---

## Quick example

Generate an animation from an input image, wait for completion, and download the result ZIP:

```bash
target/release/gametorch animations generate \
  -b \                                  # block until finished
  -i path/to/input/image.png \          # input image
  -o walking.zip \                      # where to save the ZIP
  --duration 5 \                        # optional (5 or 10 seconds)
  'walking to the left'                 # prompt
```

For additional commands and flags, run:

```bash
target/release/gametorch help
```

---

## REST API reference

The CLI is a thin wrapper over the public GameTorch REST API.  Full documentation is available at:
<https://gametorch.app/api_docs/animations>

---

## License

This project is licensed under the MIT License © GameTorch LLC.  See [LICENSE](LICENSE) for details. 