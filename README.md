<img src="https://github.com/jamesthurley/thrust/raw/main/icon.png" width="128px">

# Thrust, a theme for Rust.

A simple, clear VSCode theme with minimal distractions and tasteful orange accents (other colors are available).

Why "a theme for Rust"? 
I was writing Rust at the time, and read [this](https://rauljordan.com/rust-concepts-i-wish-i-learned-earlier/) blog post by Raul Jordan. 
The theme used in his code samples had orange accents for the keywords and operators, and I thought that worked really well. 
I couldn't find a existing similar theme for VSCode, so I decided to make one, and this is the result.

## Code Origins

The starting point for this repo was the [Pale Fire](https://github.com/matklad/pale-fire) repo by Alex Kladov.
I then customised it to create the workflow I wanted, and started playing with the colors.

Like Pale Fire, this repo uses the [Mottle](https://github.com/arzg/mottle) crate by Luna Razzaghipour to build the theme files.

## Theme Accent Colors

In addition to Thrust Orange, I created Red, Yellow and Green versions of the theme. 
This was mainly to ensure the workflow I had created was sufficiently flexible,
but if you like the others or want more colors, let me know.

## Customizing the Theme

This repository has a Dev Container configuration, so building your own `.vsix` files is as simple as
opening the repository in a VSCode Dev Container or GitHub Codespace then running ...

```
npm install
```
... to install the node dependencies (vsce), and ...

```
cargo run && npm run package
```

... to genrate the `thrust-theme-*.vsix` file.

You can then load the `.vsix` file into VSCode by opening the Extensions pane, clicking the `...` icon in the top right, and choosing "Install from VSIX". It will prompt you to reload the window to use the new extension.

As you customize the theme you can regenerate your `.vsix`, reload it in VSCode, play with the results, and iterate.

### Where to start editing code?

If you want to customise the theme at a high level, start with `src/create_palettes.rs`. 

If you want to get into the weeds, look at `src/imp.rs`.

If you want to see where the base colors are created, look at `src/colors.rs`.

### Useful Links

https://code.visualstudio.com/api/references/theme-color

https://code.visualstudio.com/api/language-extensions/syntax-highlight-guide

https://code.visualstudio.com/api/language-extensions/semantic-highlight-guide


# Themes

Currently this repository generates the following themes:
```
Thrust [Orange|Red|Yellow|Green] [Dark|Stealth|Wash]
```
## Thrust Orange Stealth Example


<img src="https://github.com/jamesthurley/thrust/raw/main/thrust-orange-stealth.png" width="803px">


## License

GPL3, like the Pale Fire repo it is based on.

## Logo

The Thrust logo was generated using [Midjourney](https://www.midjourney.com/).

<img src="https://github.com/jamesthurley/thrust/raw/main/thrust-logo.png" width="400px">
