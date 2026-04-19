# oxideav-container

Container (demuxer + muxer) traits and registry for the
[oxideav](https://github.com/OxideAV/oxideav-workspace) pure-Rust media
framework. Per-format container crates (MP4, Matroska, Ogg, WAV,
FLAC, GIF, …) implement `Demuxer` and/or `Muxer` and register via a
content-based probe so file-extension is only a tie-breaker.

* **`Demuxer`** — `next_packet`, `streams`, `seek_to`, `metadata`,
  `attached_pictures`, `duration_micros`.
* **`Muxer`** — `write_header` → `write_packet*` → `write_trailer`.
* **`ReadSeek`** / **`WriteSeek`** — the I/O abstraction demuxers and
  muxers operate on (a pluggable `Box<dyn ReadSeek + Send>`).
* **`ContainerRegistry`** — opens input by probing the first 256 KB,
  chooses the highest-scoring demuxer; writes output via an
  extension-indexed muxer factory.

Zero C dependencies. Zero FFI.

## Usage

```toml
[dependencies]
oxideav-container = "0.1"
```

## License

MIT — see [LICENSE](LICENSE).
