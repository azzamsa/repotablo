<div align="center">
<h1>repotablo</h1>

<a href="https://github.com/azzamsa/repotablo/actions/workflows/ci.yml">
  <img src="https://github.com/azzamsa/repotablo/actions/workflows/ci.yml/badge.svg">
</a>
<a href="https://crates.io/crates/repotablo">
  <img src="https://img.shields.io/crates/v/repotablo.svg">
</a>
<a href="https://docs.rs/repotablo/">
  <img src="https://docs.rs/repotablo/badge.svg">
</a>
<a href="https://azzamsa.com/support/"><img alt="Sponsor me" src="https://img.shields.io/badge/Sponsor%20Me-%F0%9F%92%96-ff69b4"></a>
<p></p>

<img width="600" alt="Demo" src="https://github.com/user-attachments/assets/786df3f7-a461-4258-9a57-43ffdb7a7ce2" />

</div>

---

**repotablo** is a CLI tool that helps you discover high-quality repositories faster.

## Features

- Sortable columns
- Repository filtering
- Read input from `$EDITOR`, a local file, or a remote URL
- Colorized popularity and maintenance scores
- Copy to clipboard and open repositories in your browser
- Exclude repositories below a minimum star threshold
- Export results to Markdown
- View detailed repository information
- Cross-platform support

## Why?

When browsing awesome lists, I often open dozens of tabs to manually compare repositories. It’s slow and repetitive.\
**repotablo** streamlines that process so you can evaluate and rank repositories directly from the terminal.

## Usage

```bash
repotablo                                  # Open $EDITOR to paste your repo list
repotablo input.md                         # Read from a local file
repotablo https://raw.../../README.md      # Read from a remote file
repotablo --min-stars 1000                 # Exclude repos with fewer than 1000 stars
```

You can use repotablo without a GitHub token, but you may encounter rate limits.
To authenticate:

```bash
export GITHUB_TOKEN=...
```

## Installation

### From binaries

The [release page](https://github.com/azzamsa/repotablo/releases) includes
pre-compiled binaries for GNU/Linux, macOS and Windows.

### From source

Using [cargo-binstall](https://github.com/cargo-bins/cargo-binstall)

```bash
cargo binstall repotablo
```
