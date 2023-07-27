
# Thrust, a theme for Rust.
<img src="https://github.com/jamesthurley/thrust/raw/main/icon.png" width="64px">

A VSCode theme with minimal distractions and tasteful orange accents (other colors are available).

Thrust's colors and font styles are designed to be minimal and consistent. 
Many themes use large and varied color palettes, leading to information overload. 
Thrust uses a minimal one, making the code the focus and not the theme itself.

Despite this, by carefully choosing the colors and font styles, it intuitively conveys a large amount of information.

 - **Variables**, **fields**, and **parameters** use a light gray, almost white, making them easy to pick out and read.
 - **Namespaces** and **macros** use slightly darker grays. They aren't a distraction, but are subtly differentiated.
 - **Functions** take on some of the accent color, again differentiating them without drawing too much attention.
 - **Keywords** and **operators** use the accent color, which by default is orange. These punctuate your code, helping break it up.
 - **Types** and **type parameters** are a color complementary to the accent color, which by default is a tasteful blue.
 - **User defined types** are bolded, drawing more attention to them.
 - **Traits**, **interfaces**, and **generic type parameters** are italicized, indicating their more abstract nature.
 - **Constants** are a darker blue, **strings** a lighter blue, **numbers** a complementary salmon pink.

It is recommended to turn off bracket colorization when using Thrust to minimize color distractions (you can always toggle
them back on when you need them, using an extension such as `Toggle`).

## Why "a theme for Rust"? 
Despite the name, the theme should work well in any language, I personally use it for C#, TypeScript, Python, and, of course, Rust.

I was writing Rust at the time, and read [this blog post](https://rauljordan.com/rust-concepts-i-wish-i-learned-earlier/) by Raul Jordan. 
In that blog post's code samples, I was drawn to the orange accents used for the keywords and operators, and the otherwise 
minimal use of color.
The orange accents felt particularly well suited to the Rust language.

I was also fed up by with the multitude of arbitrary colors used by other themes which distracted from the code itself,
and wanted to carefully craft my own theme with more intuitive colors and font styles.


# Theme Variants

Currently this repository generates the following theme variants:
```
Thrust [Orange|Red|Yellow|Green] [Dark|Stealth|Wash]
```
## Thrust Orange Stealth Example

<img src="https://github.com/jamesthurley/thrust/raw/main/thrust-orange-stealth.png" width="803px">

## Code Origins

The starting point for this repo was the [Pale Fire](https://github.com/matklad/pale-fire) repo by Alex Kladov.
I then customized it to create the workflow I wanted, and started playing with the colors.

Like Pale Fire, this repo uses the [Mottle](https://github.com/arzg/mottle) crate by Luna Razzaghipour to build the theme files.

## Theme Accent Colors

In addition to Thrust Orange, I created Red, Yellow and Green versions of the theme. 
This was mainly to ensure the workflow I had created was sufficiently flexible at creating variations on the primary theme,
but if you like the others or want more options, let me know.

## Customizing the Theme

This repository has a Dev Container configuration, so building your own `.vsix` files is as simple as
opening the repository in a VSCode Dev Container or GitHub Codespace then running ...

```
npm install
```
... to install the node dependencies (`vsce`), and ...

```
cargo run && npm run package
```

... to generate the `thrust-theme-*.vsix` file.

You can then load the `.vsix` file into VSCode by opening the Extensions pane, clicking the `...` icon in the top right, and choosing "Install from VSIX". It will prompt you to reload the window to use the new extension.

As you customize the theme you can regenerate your `.vsix`, reload it in VSCode, play with the results, and iterate.

### Where to start editing code?

If you want to customize the theme at a high level, start with `src/create_palettes.rs`. 

If you want to get into the weeds, look at `src/imp.rs`.

If you want to see where the base colors are created, look at `src/colors.rs`.

### Publishing

The repository is set up so that running ...
```
npm version patch
```
... will create a new minor version of the theme,
run the tests, and push it.

The GitHub actions are then configured to publish the new version to the VSCode marketplace.

### Useful Links

https://code.visualstudio.com/api/references/theme-color

https://code.visualstudio.com/api/language-extensions/syntax-highlight-guide

https://code.visualstudio.com/api/language-extensions/semantic-highlight-guide


## Logo

The Thrust logo was generated using [Midjourney](https://www.midjourney.com/).

<img src="https://github.com/jamesthurley/thrust/raw/main/thrust-logo.png" width="400px">

## License

GPL3, like the Pale Fire repo it is based on.
