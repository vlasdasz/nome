# nome

A metronome. One Rust crate on [test-engine](https://github.com/hilen/test-engine), so the
same code runs on desktop, iOS and Android. The beat playback comes from
[mnomer](https://github.com/VladasZ/mnomer).

## Run

```bash
make run        # desktop
make ios        # generate the Xcode project and build
make android    # android build
```

## Checks

```bash
make ci         # typos, fmt, clippy, machete
make lint       # clippy only
```
