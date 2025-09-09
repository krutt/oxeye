# Oxeye

[![Bitcoin-only](https://img.shields.io/badge/bitcoin-only-FF9900?logo=bitcoin)](https://twentyone.world)
[![Top](https://img.shields.io/github/languages/top/krutt/oxeye)](https://github.com/krutt/oxeye)
[![Languages](https://img.shields.io/github/languages/count/krutt/oxeye)](https://github.com/krutt/oxeye)
[![Last commit](https://img.shields.io/github/last-commit/krutt/oxeye/master)](https://github.com/krutt/oxeye)
![Oxeye Banner](static/oxeye-banner.svg)

### Prerequisites

* [python](https://www.python.org) 3.9 and above - High-level general-purpose programming language
* [pip](https://pypi.org/project/pip) - package installer for Python

### Getting started

```sh
pip install oxeye
```

And then you can start an LSP instance with the following command:

```sh
oxeye
```

Or use one of the following plugins on your desired Editor or Integrated Development Environment (IDE)

* Neovim: [oxeye.nvim](https://github.com/krutt/oxeye.nvim)
* ฯลฯ

## Contributions

    oxeye/
    │
    ├── src/
    │   └── lib.rs               # Library definitions
    │
    ├── static/
    │   ├── docs/                # Collection of markdown LSP definitions
    │   │   ├── comprehensive/   # Collection of catchall markdown reference
    │   │   ├── jet_functions/   # Collection of markdown jet function definitions
    │   │   ├── keywords/        # Collection of markdown keyword definitions
    │   │   ├── misc/            # Collection of miscellaneous definitions
    │   │   └── types/           # Collection of type definitions
    │   │
    │   ├── oxeye.svg            # Vector asset used as crest
    │   ├── oxeye-banner.svg     # Vector asset for banner on display in README
    │   └── oxeye-social.svg     # Vector asset used as Open Graph preview
    │
    ├── Cargo.toml               # Rust library dependencies and packaging
    ├── LICENSE                  # Details of MIT License
    ├── README.md                # Descriptions and roadmap
    ├── oxeye.pyi                # Stub file for python method declarations
    └── pyproject.toml           # Python library build system and metadata

  > :see_no_evil:
  > Notable exemptions: `example.simplicity`, `formatting rules`, `gitignore` and `lockfiles`

### Prerequisites

* [git](https://git-scm.com/) - --fast-version-control
* [uv](https://docs.astral.sh/uv) - Extremely fast Python package & project manager, written in Rust
* [rustup](https://rustup.rs) - An installer for the systems programming language Rust
* [maturin](https://github.com/PyO3/maturin) - Build and publish creates with pyo3, cffi and uniffi
  bindings as well as rust binaries as python packages


<details>
  <summary> Environment setup guide </summary>

  The following guide walks through setting up your local working environment using `git`
  as distributed version control system, `uv` as Python package and version manager,
  `rustup` toolchain installer for Rust programming language and `maturin` buildtools for
  python packages using Rust. If you do not have `git` installed, run the following command.
  
  <details>
    <summary> Install using Homebrew (Darwin) </summary>
    
    brew install git
  </details>
  
  <details>
    <summary> Install via binary installer (Linux or Windows Subsystem for Linux [WSL]) </summary>
    
  * Debian-based package management

        sudo apt install git-all
  
  * Fedora-based package management

        sudo dnf install git-all
  </details>
  
  <details>
    <summary> Install using Winget (Windows Powershell) </summary>
    
    winget install --id Git.Git -e --source winget
  </details>
  
  If you do not have `uv` installed, run the following command.
  
  <details>
    <summary> Install using Homebrew (Darwin) </summary>
  
    brew install uv
  </details>
  
  <details>
    <summary>
      Install using standalone installer (Darwin, Linux, or Windows Subsystem for Linux [WSL])
    </summary>
  
    curl -LsSf https://astral.sh/uv/install.sh | sh
  </details>
  
  <details>
    <summary> Install using Winget (Windows Powershell) </summary>
  
    winget install --id=astral-sh.uv -e
  </details>
  
  If you do not have `rustup` installed, run the following command.
  
  <details>
    <summary>
      Install using Homebrew (Darwin)
    </summary>
  
    brew install rustup
  </details>
  <details>
    <summary>
      Install using standalone installer (Darwin, Linux or Windows Subsystem for Linux [WSL])
    </summary>
  
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  </details>
  <details>
    <summary> Install using Winget (Windows Powershell) </summary>
    
    winget install --id Rustlang.Rustup
  </details>
  
  After running `rustup` in your command line, you should be able to validate correct installations
  as such:
  
  ```sh
  rustc --version
  cargo --version
  ```
  
  If you do not have `maturin` installed, run the following command.
  
  <details>
    <summary> Install using Homebrew (Darwin) </summary>
  
    brew install maturin
  </details>
  <details>
    <summary>
      Build from Source using cargo (Linux, Powershell and Windows Subsystem for Linux [WSL])
    </summary>
  
    cargo install --locked maturin
  </details>
  
  Once you have all tools installed, you can clone the current repository and
  install any version of Python above version 3.9 for this project. The following
  commands help you set up and activate a Python virtual environment where `uv`
  can download project dependencies from the `PyPI` open-sourced registry defined
  under `pyproject.toml` file.
  
  <details>
    <summary> Set up environment and synchronize project dependencies </summary>
  
    git clone git@github.com:krutt/oxeye.git
    cd oxeye
    uv venv --python 3.9.6
    source .venv/bin/activate
    uv sync --dev --seed
  </details>
</details>

## Roadmap

* Cross-check and update language references
* Implement Go to definitions

## Acknowledgements

1. [อัญชัญ - Anchan](https://www.f0nt.com/release/anchan/) typeface by [Alisara Zilch](https://www.f0nt.com/author/zilch/)
2. [BlockStreamResearch/SimplicityHL](https://github.com/BlockStreamResearch/SimplicityHL) -
  Rust-like high-level language that compiles down to Simplicity bytecode. Work in progress.
3. [BlockStreamResearch/simplicity](https://github.com/BlockStreamResearch/simplicity) -
  Simplicity is a blockchain programming language designed as an alternative to Bitcoin script.
4. [BlockStreamResearch/rust-simplicity](https://github.com/BlockStreamResearch/rust-simplicity) -
  Official [Rust](https://www.rust-lang.org) library for [Simplicity language](https://simplicity-lang.org)
5. [Simplicity: Next-Generation Smart Contracts for Bitcoin](https://simplicity-lang.org) -
  Developers write SimplicityHL, full nodes execute Simplicity.
6. [Tower LSP](https://github.com/ebkalderon/tower-lsp) -
  Language Server Protocol implementation written in Rust 

## License

This project is licensed under the terms of the MIT license.
