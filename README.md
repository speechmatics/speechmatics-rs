# Speechmatics ❤️  🦀🦀 Rust 🦀🦀

![Speechmatics Rust](https://github.com/speechmatics/speechmatics-rs/tree/main/workflows/test.yml)


Here at Speechmatics we love rust. That's why we've given you not one, but TWO rust packages:

1. speechmatics is a 100% pure synchronous, single-threaded implementation of our Realtime API. Its performance from a turn-around time perspective is worse than the async implementation, but if you care about keeping depedencies to a minimum or otherwise want to stay away from async runtimes, this package is going to be for you.

2. speechmatics-async (WIP) is, you guessed it, our async API. Specifically, we expose an async realtime API based on [tokio-tungstenite](https://docs.rs/tokio-tungstenite/latest/tokio_tungstenite/) and a batch API based on [Reqwest](https://docs.rs/reqwest/latest/reqwest/). In order to enable these features, you will need to add feature flags for `rt` and `batch` respectively

You can find our more about each of these packages in their respective readmes.