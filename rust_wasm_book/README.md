.wat is webassembly text
.wasm is binary webassembly

Asks you to install `wasm-pack` and `cargo-generate`.
`cargo-generate` appears to be a way to set up a template repository.

## Hello, World!

The templete used was `cargo generate --git https://github.com/rustwasm/wasm-pack-template`.
It's _fast_, I hit enter and when I moved my eyes to the folder tree in VSCode, everything was already there.
This template appears to have lots of stuff I probably won't use (like travis and appveyor config files. I'll use Github actions if needed).

It sets up a bunch of things, like a `Cargo.toml` loaded with needed config.
One of the most important dependencies there is `wasm-bindgen`.

`lib.rs` is where most of the Rust happens.
It uses `wasm-bindgen` to bridge with JavaScript.
It sets up an external function called `alert`, that's the one from the browser.
If we call it here, the browser function will be called.

When running `wasm-pack build` in that generated directory I got an error.

```
warning: be sure to add `/home/nicky/.cache/.wasm-pack/.wasm-bindgen-cargo-install-0.2.78/bin` to your PATH to be able to run the installed binaries
Error: failed to download from https://github.com/WebAssembly/binaryen/releases/download/version_90/binaryen-version_90-x86-linux.tar.gz
To disable `wasm-opt`, add `wasm-opt = false` to your package metadata in your `Cargo.toml`.
```

Apparently the "fix" is to disable optimization.
That's kinda unacceptable and defats the point, but if it'll let me move on from the "hello-world" example, sure.
Adding this to `Cargo.toml`

```
# https://github.com/rustwasm/wasm-pack/issues/864#issuecomment-647850921
[package.metadata.wasm-pack.profile.release]
wasm-opt = false
```

That created a `pkg/` directory with a `package.json`, readme, a `.wasm`, `.ts`, and `.js` file.

The `.wasm` file is binary, so not human readable.

The `.js` contains glue for getting JavaScript things into Rust,
and letting the WASM functions be called from JS.
It imports things from that `.wasm` file.

For example, that `greet` function, it has an exported `greet` functions that wraps a call to the wasm `greet` function.

The `package.json` holds some metadata that's used by npm and js bundlers.

### Putting it into a web page

To put it in a webpage, use [`create-wasm-app`](https://github.com/rustwasm/create-wasm-app).

```
npm init wasm-app www
```

That created a `www/` folder.

That sets up a basic site with the necessary plumbing.
It uses webpack.
The WASM is now an npm package somehow.
That package has to be dynamically imported which is done in `bootstrap.js`.
That file is also the one inside the `<script src="./bootstrap.js">` tag in the HTML.

That `hello-wasm-pack` package contains the WASM code and the JS glue.
It is imported and used in `index.js` (which in turn is dynamically imported in `bootstrap.js`).

That npm package is imported with `* as wasm` in `index.js`.
Then the glue JS `greet` function is called with `wasm.greet()`.

`npm run start` boots the webpack server, and a site with an alert box pops up.
That box says `Hello wasm-pack` for now because that npm package we used is not our local one yet.

#### Using our local package

Instead of using that `hello-wasm-pack` package from npm, we want to use our local `wasm-game-of-life` one.

This is done by adding it in the www folder's package.json

```json
"dependencies": {
    "wasm-game-of-life": "file:../pkg"
},
```

Run `npm install` again and replace the import in `index.js`.

```js
import * as wasm from "wasm-game-of-life";
```

Restarting the webpack server with `npm run start` now opens a page with the message from `lib.rs`.
Rust to WASM, baby!

Chaging stuff in `lib.rs` doesn't automatically result in changes in that webpage.
The webpage pulls from that local package, so that has to be remade before a change in Rust takes effect.

So `wasm-pack build` has to be ran if changes are made in Rust.

Remember, `wasm-pack build` is ran in `wasm-game-of-life/`.
`npm run start` is ran in `wasm-game-of-life/www`.