# Madara CLI

A command-line tool for rapid deployment of Madara app chains.

## Dependencies

There are a few dependencies that need to be installed to smoothly `madara-cli`.

[Installing dependencies](./docs/setup.md)

## Quick Start

- Clone repo:

```bash
git clone https://github.com/karnotxyz/madara-cli
```

- Build repo:

```bash
cd madara-cli
cargo build --release
```

- Initialize a new app chain. Please fund the DA account (if applicable):

```bash
./target/release/madara init
```

- Run your app chain:

```bash
./target/release/madara run
```

- Optionally, explore the StarkScan explorer. Accessible at [http://localhost:4000](http://localhost:4000).

```bash
./target/release/madara explorer
```

**Congratulations! You now have a custom madara app running.**
