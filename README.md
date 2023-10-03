# Speechmatics Rust

Speechmatics ❤️  🦀🦀 Rust 🦀🦀! That's why we've given you not one, but TWO rust crates:

1. speechmatics is a 100% pure synchronous, single-threaded implementation of our Realtime API. Its performance from a turn-around time perspective is worse than the async implementation, but if you care about keeping depedencies to a minimum or otherwise want to stay away from async runtimes, this package is going to be for you.

2. speechmatics-async is our async API. Specifically, we expose an async realtime API based on [tokio-tungstenite](https://docs.rs/tokio-tungstenite/latest/tokio_tungstenite/) and a batch API based on [Reqwest](https://docs.rs/reqwest/latest/reqwest/). In order to enable these features, you will need to add feature flags for `rt` and `batch` respectively. Note that the realtime features are only compatible with tokio - but the batch features should work for any async runtime.

You can find our more about each of these packages in their respective readmes.
- [speechmatics](./speechmatics/README.md)
- [speechmatics-async](./speechmatics-async/README.md)