# kubectx

[![Build Status](https://travis-ci.org/FGRibreau/kubectx-rs.svg?branch=master)](https://travis-ci.org/FGRibreau/kubectx-rs) [![Build status](https://ci.appveyor.com/api/projects/status/9xsw4bboduma93tv/branch/master?svg=true)](https://ci.appveyor.com/project/FGRibreau/kubectx-rs/branch/master) [![Cargo version](https://img.shields.io/crates/v/kubectx.svg)](https://crates.io/crates/kubectx) [![Crates.io](https://img.shields.io/crates/d/kubectx.svg)](https://crates.io/crates/kubectx) [![Slack](https://img.shields.io/badge/Slack-Join%20our%20tech%20community-17202A?logo=slack)](https://join.slack.com/t/fgribreau/shared_invite/zt-edpjwt2t-Zh39mDUMNQ0QOr9qOj~jrg)


Fastest switch between Kubernetes contexts 🏎.

![kubectx](/docs/kubectx.gif)


### [⭐️ Download pre-built binaries ⭐️](https://github.com/FGRibreau/kubectx-rs/releases)

### Installation

```shell
cargo install kubectx
```

### Usage

```sh
$ kubectx
--Select kubectl context (Use arrow keys)
  image-charts-production
❯ killbug-development
  minikube
  ouest-france-development
```

#### Next steps

- [ ] support namespace switch as well (kubens)

#### Related work

- kubectx is completly inspired by [kubectx](https://github.com/ahmetb/kubectx) from Ahmet Alp Balkan. I personally prefer the interactive way that's why this project exist :).
- before this Rust version, [kubectx was written in JavaScript with NodeJS](https://github.com/FGRibreau/kubectx).
