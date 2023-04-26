# 🪙 Coinbase Alfred Workflow

A simple [Alfred Workflow](https://www.alfredapp.com/workflows/) to check the current _spot_ prices of some of the top cryptocurrencies.

<img style="padding: 1rem 0" width="605" alt="Screenshot" src="https://user-images.githubusercontent.com/5381156/234696277-3219989d-15e9-47f7-b0c4-dd54bbae7525.gif">

> Big thank you to [@rossmacarthur](https://github.com/rossmacarthur) for creating [powerpack ⚡️](https://github.com/rossmacarthur/powerpack) 👏🏽

Supported coins:

- Bitcoin
- Cardano
- DogeCoin
- Ethereum
- Litecoin
- Polkadot
- Polygon
- Solana

## 🌈 Features

- Check the spot price of some of the top 9 on coinbase.
- Fuzzy find.
- Open coinbase to the coin in the default browser.

## 📦 Installation

### Pre-packaged

Grab the latest release from
[the releases page](https://github.com/derekahn/coinbase.alfredworkflow/releases).

Because the release contains an executable binary later versions of macOS will mark it as untrusted.
You can run the following to explicitly trust the release before installing to Alfred.

```bash
xattr -c ~/Downloads/coinbase-*-x86_64-apple-darwin.alfredworkflow
```

### Building from source

This workflow is written in Rust, so to install it from source you will first
need to install Rust and Cargo using [rustup](https://rustup.rs/). Then install
[powerpack](https://github.com/rossmacarthur/powerpack). Then you can run the
following to build an `.alfredworkflow` file.

```bash
git clone https://github.com/derekahn/coinbase.alfredworkflow.git

cd coinbase.alfredworkflow

powerpack package
```

The release will be available at `target/workflow/coinbase.alfredworkflow`.

## 🚧 Roadmap

- [ ] configurable coin selection
- [ ] configurable currency selection (other than USD)

## 🪪 License

This project is distributed under the terms of the MIT license.
