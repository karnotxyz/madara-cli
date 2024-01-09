# Installing Dependencies

## Install `Git`

Install the latest `git` version

Instruction can be found on the [official site](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git).

Verify the `git` installation:

```bash
git --version
git version 2.34.1
```

## `Install Rust`

Install the latest `rust` version.

Instructions can be found on the [official site](https://www.rust-lang.org/tools/install).

Verify the `rust` installation:

```bash
rustc --version
rustc 1.75.0 (82e1608df 2023-12-21)
```

## `Docker`

Install `docker`. It is recommended to follow the instructions from the
[official site](https://docs.docker.com/install/).

Installing `docker` via `snap` or from the default repository can cause troubles.

**Note:** On linux you may encounter the following error when you’ll try to work with `madara-cli`:

```bash
ERROR: Couldn't connect to Docker daemon - you might need to run `docker-machine start default`.
```

If so, you **do not need** to install `docker-machine`. Most probably, it means that your user is not added to
the`docker` group. You can check it as follows:

```bash
docker-compose up # Should raise the same error.
sudo docker-compose up # Should start doing things.
```

If the first command fails, but the second succeeds, then you need to add your user to the `docker` group:

```bash
sudo usermod -a -G docker your_user_name
```

After that, you should logout and login again (user groups are refreshed after the login). The problem should be
solved at this step.

If logging out does not help, restarting the computer should.

**Additionally, there are a few more dependencies required by madara for building various crates and related dependencies.**.

## `Ubuntu`

These are only required on ubuntu systems.

```bash
sudo apt update
sudo apt upgrade
sudo apt install build-essential

sudo apt install pkg-config
sudo apt install libssl-dev
sudo apt install clang
sudo apt install protobuf-compiler
```

## `MacOS`

Verify `brew` installation:

```bash
brew --version
Homebrew 4.2.2
```

If brew is not found, install using following command:

```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

Then install the following:

```bash
brew install pkg-config
brew install openssl
brew install protobuf
```

**Make sure to follow these instructions to ensure a smooth setup for `madara`.**
