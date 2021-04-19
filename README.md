# Rust + ReScript Demo

Demo app built with [Rust](https://www.rust-lang.org) and [ReScript](https://rescript-lang.org).

## TOC
- [Tech Stack](#tech-stack)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
- [Developing](#developing)
  - [Windows Support](#windows-support)
- [Not Implemented Yet](#not-implemented-yet)

## Tech Stack
API server:
- [Rust](https://www.rust-lang.org)
- [Actix](https://actix.rs)
- [SQLx](https://github.com/launchbadge/sqlx)
- [GraphQL](https://github.com/async-graphql/async-graphql)

Web client:
- [ReScript](https://rescript-lang.org)
- [ReasonReact](https://reasonml.github.io/reason-react/)

## Getting Started

### Prerequisites
Before proceeding, make sure you have:
- [Rust toolchain](https://www.rust-lang.org/tools/install)
- [Docker and Docker Compose](https://www.docker.com/get-started)

### Installation
Mostly everything in this project is managed via its own CLI.

To install the project's CLI, run in the root directory of this repository:

```sh
cargo run cli install
```

After that, you should have an `rrd` binary available in your shell.

To setup a local environment, run:

```sh
rrd setup
```

That's it. You are ready to roll.

---
If `rrd` command is not available in your shell, make sure your [Rust toolchain is properly configured](https://www.rust-lang.org/tools/install). Especially, check that Cargo's bin directory is in your `PATH`.

## Developing
To start developing the app, run:

```sh
rrd develop
```

To explore what commands are available, run `rrd help`.

### Windows Support
Windows should be supported. Though I'm totally unfamiliar with this operating system and some bits of the CLI most likely must be refined to fully support all features on Windows. Appreciate [issues](https://github.com/shakacode/rust-rescript-demo/issues/new)/[pull requests](https://github.com/shakacode/rust-rescript-demo/compare) if something doesn't work.

## Not Implemented Yet
This project is quite raw and missing a bunch of pieces.

Some of them off the top of my head:
- No tests
- No i18n
- No validations: neither server-side nor client-side
- No caching mechanism in the web client

I'm sure there's a lot more. Feel free to [create an issue](https://github.com/shakacode/rust-rescript-demo/issues/new) if you are interested in something specific.
