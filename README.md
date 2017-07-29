# kubectx

[![Build Status](https://travis-ci.org/FGRibreau/kubectx-rs.svg?branch=master)](https://travis-ci.org/FGRibreau/kubectx-rs) [![Build status](https://ci.appveyor.com/api/projects/status/9xsw4bboduma93tv/branch/master?svg=true)](https://ci.appveyor.com/project/FGRibreau/kubectx-rs/branch/master)


Fastest switch between Kubernetes contexts 🏎.

![kubectx](/docs/kubectx.gif)

### Installation

```bash
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
- before this Rust version, [kubectx was wrote over NodeJS](https://github.com/FGRibreau/kubectx).
