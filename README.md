# {{ project-name }}

{% comment %} 

```sh
# needs cargo-generate 0.6.0.alpha.1
cargo install cargo-generate --version 0.6.0-alpha.1

## generate the template 
cargo generate --git sassman/hermit-template-rs
```

{% endcomment %}

## Rusty Loader

This template assumes:
- [rusty-loader](https://github.com/hermitcore/rusty-loader) checked out at `../rusty-loader`
- rusty-loader was build as release with `release=1 make` 