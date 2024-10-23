<br />
<p align="center">
  <a href="https://arbitrum.io/">
    <img src="https://arbitrum.io/assets/stylus/stylus_with_paint_bg.png" alt="Logo" width="100%">
  </a>

  <h3 align="center">The Stylus SDK</h3>

  <p align="center">
    <a href="https://docs.arbitrum.io/stylus/stylus-gentle-introduction"><strong>Rust contracts on Arbitrum »</strong></a>
    <br />
  </p>
</p>

## Overview

The Stylus SDK enables smart contract developers to write programs for **Arbitrum chains** written in the [Rust](https://www.rust-lang.org/tools/install) programming language. Stylus programs are compiled to [WebAssembly](https://webassembly.org/) and can then be deployed on-chain to execute alongside Solidity smart contracts. Stylus programs are not only orders of magnitude cheaper and faster but also enable what was thought to be previously impossible for WebAssembly: **EVM-interoperability**.

For information about deploying Rust smart contracts, see the [Cargo Stylus CLI Tool](https://github.com/OffchainLabs/cargo-stylus). For more information about Stylus, see [Stylus: A Gentle Introduction](https://docs.arbitrum.io/stylus/stylus-gentle-introduction). For a simpler intro to Stylus Rust development, see the [Quick Start guide](https://docs.arbitrum.io/stylus/stylus-quickstart).

## Feature highlights

The SDK makes it easy to develop Ethereum ABI-equivalent Stylus contracts in Rust. It provides a full suite of types and shortcuts that abstract away the details of Ethereum's storage layout, making it easy to _just write Rust_. For an in depth exploration of the features, please see comprehensive [Feature Overview][overview].

Some of the features available in the SDK include:

- **Generic**, storage-backed Rust types for programming **Solidity-equivalent** smart contracts with optimal storage caching.
- Simple macros for writing **language-agnostic** methods and entrypoints.
- Automatic export of Solidity interfaces for interoperability across programming languages.
- Powerful **primitive types** backed by the feature-rich Alloy.

Rust programs written with the Stylus SDK can call and be called by Solidity smart contracts due to ABI equivalence with Ethereum programming languages. In fact, existing Solidity DEXs can list Rust tokens without modification, and vice versa.
