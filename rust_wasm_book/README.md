Book at https://rustwasm.github.io/docs/book/

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

---
I reinstalled wasm-pack today, just because _C O M P U T E R S_.
Uncommented the config in `Cargo.toml` that disabled `wasm-opt`.

What do you know, it works now.
I have no idea why, I hate this.
My best guess is WSL was missing some necessary plumbing wasm-opt relies on that got installed too with
```
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```
---

## Implementing Conway's Game of Life

The game is played on an infinite plane.
Computers don't have infinite memory, so this guide chooses the field to be finite, and wrap around (off right edge, to left edge etc).

### Interfacing Rust and JavaScript

The self proclaimed "most important concepts of the book" are explained here, better pay attention.

JavaScript's memory heap (that is garbage collected) is seperate from WebAssembly's linear memory, where our Rust stuff lives.

- WASM has no direct access to that garbage collected heap (expected to change SOON:tm:).
- JavaScript can read and write to the WASM linear memory space

The way JS can read/write that memory is with an `ArrayBuffer` of scalar values (so, numbers).
WASM functions take and return scalar values.

Nicky: The things that get sent across the JS/WASM boundary are pointers to these scalar values.

`wasm-bindgen` defines defines a common way to communicate across this boundary.
It boxes Rust structs and wraps the pointer in a JavaScript class.
Or it can index into a table of JS objects from Rust.

Nicky: We write familiar Rust/JS data structures and wasm-bindgen converts it to those scalar values so JS/Rust can read what happened on the other side of the Rust/JS boundary.

For communication between WASM and JS, we want to optimize for a few things.
1. Minimizing copying into and out of the WebAssembly linear memory.
2. Minimizing serializing and deserializing.

Instead of serializing on one side, just to deserialize on the other.
Many times we can pass opaque handles (what does opaque mean in this context, smart pointer?) across the boundary instead, avoiding that work.
`wasm-bindgen` helps us define and work with opaque handles to JavaScript Objects or boxed Rust structures.

Nicky: Minimize work, then things go faster. Radical idea, right?

As a general rule of thumb, a good JavaScript↔WebAssembly interface design is often one where large, long-lived data structures are implemented as Rust types that live in the WebAssembly linear memory, and are exposed to JavaScript as opaque handles. 
JavaScript calls exported WebAssembly functions that take these opaque handles, transform their data, perform heavy computations, query the data, and ultimately return a small, copy-able result.

So, we don't want to copy the entire game of life universe into and out of WASM linear memory on every tick.
We don't want to allocate objects for every cell in the universe.

We can represent the (2D) universe as a flat array that lives in the WASM linear memory, the 2 states for a cell are represented by either a 0 or 1.
One row laid out after another, no 2D array here, just a continuous array.

We can calculate the index of a cell in this array with the row and column nr.
```
index(row, column, universe) = row * width(universe) + column
```

To start, we implement `std::fmt::Display` for `Universe`.
This generates a Rust `String` that can be copied from the WASM linear memory into a JavaScript string that is stored in the JS garbage-collected heap.
Then it can be set in HTML via the `textContent` method on a DOM element.

The tutorial later changes this implementation to avoid copying between heaps and renders to a `<canvas>`.

### Rust implementation

Make a `Cell` enum

```rust
#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1
}
```

The `repr` attribute makes sure each cell is represented by a single byte.
We set `Dead = 1`, and `Alive = 1` so we can count the number of alive neighbours by addition.

The `Universe` struct has a height, a width, and a vector of cells.
(that vector's length is width * height)
```rust
#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}
```

We implement a `get_index` method on the Univrse struct that takes a row and a col num, and return an index into the cells array at that point.

We implement a `live_neighbour_count` function that count the number of neighbours that are alive with that wrapping logic.

We implement a `tick` function that calculates the new cells in the universe and sets them.
Because we want JS to call this function, we add `#[wasm_bindgen]` to the `impl` block for `Universe`, and make the `tick` method `pub`.

We implement `Display` trait on `Universe` to get a human readable output of the `cells` vector.

We implement a `pub` `new` associated function on `Universe` to generate an initial universe with a cool pattern.
It's inside the `impl Universe` which has the `#[wasm_bindgen]` attribute, so it is exposed to JavaScript.

We implement a `pub render` method that returns the stringified universe via the `to_string` method we implemented through adding the `Display` trait.

### Rendering with JS

We add a `pre` tag to `index.html` that'll get populated.
Some styles to make it look less plain.

In the `index.js` file in `www/` we now import our `Universe` struct from Rust, that WASM turned into a JS object we can use.

It still has the methods we defined in Rust though, so we create a new instance by calling the `new` method on the `Universe` we imported.

```js
import { Universe } from "wasm-game-of-life";
const pre = document.getElementById("game-of-life-canvas");
const universe = Universe.new();

const renderLoop = () => {
  pre.textContent = universe.render();
  universe.tick();

  requestAnimationFrame(renderLoop);
};
requestAnimationFrame(renderLoop);
```

Rerun wasm-pack build, rerun npm run start, boom a game of life.

### Rendering to canvas directly from memory

Making a `String` in Rust (and storing it in the WASM linear memory) and then having wasm-bindgen convert it to a javascript string (which is stored in the JS memory) is expensive.
The same thing is stored in 2 locations.
It makes unnecessary copies of the universe's cells.

As JS can read the WASM memory, we'll modify our `render` method to return a pointer to the start of the cells array.
At the same time, we'll switch to a `<canvas>` HTML element to render the game of life.

To get the needed info into JS, we add a width and heigh getter in `lib.rs`.
We add a `cells` method that returns a pointer to the `cells` vector.

The `cells` method returns a raw pointer to the cells vector buffer, a `*const Cell`.
This kinda scares me, as raw pointers scream DANGER to me.
https://doc.rust-lang.org/std/vec/struct.Vec.html#method.as_ptr
But apparently it being WASM ensures some of the invariants that have to be upheld (like the memory it points to not disappearing).

If I understand correctly, that pointer points to the location in  memory where the cells vector starts, that's why it's `*const Cell`, because that vector is filled with `Cell`s.

We change `<pre>` to `<canvas>`, access that DOM element in `index.js`.
Set a width and height by calling the functions on `Universe`.

Set the canvas size according to those values.
Draw a grid on the canvas.
Draw the cells on the canvas.
Each render loop consists of those steps.
1. calculate new state
2. draw grid
3. draw cells

The drawing of the cells uses direct access of the WebAssembly linear memory.
That is defined in that raw WASM module `wasm_game_of_life_bg` that `wasm-pack` generated inside the `wasm-game-of-life/pkg/` folder.
We can access the memory directly by importing it in our `index.js`.

We import `memory` from `wasm-game-of-life/wasm_game_of_life_bg`.
Nicky: is that the `.wasm` file or the `.js` file with that name?
I guess `.wasm` since I can't find a `memory` export from the js file.

```js
// Import the WebAssembly memory at the top of the file.
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

// ...

const getIndex = (row, column) => {
  return row * width + column;
};

const drawCells = () => {
  const cellsPtr = universe.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

  ctx.beginPath();

  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col);

      ctx.fillStyle = cells[idx] === Cell.Dead
        ? DEAD_COLOR
        : ALIVE_COLOR;

      ctx.fillRect(
        col * (CELL_SIZE + 1) + 1,
        row * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }

  ctx.stroke();
};
```

We get a pointer the the universe's cells via the `universe.cells()` method we defined in `lib.rs`.
We construct a `Uint8Array` in our `index.js` with that pointer combined with the height and width.
Remember, the cells vector is width * height long.
Each item is a `Cell`, and we know each of those is exactly 1 byte big because we set `#[repr(u8)]`.

We iterate over each cell that way, and render a square on the corresponding location on the canvas with the alive or dead color.

By doing this we avoided copying so much memory across the boundary and storing the same thing in 2 locations.
Now, instead of the whole `String` representation of the `Universe` being copied to a JS string, we clone the pointer to the exact point in WASM memory and iterate over the cells by knowing exactly how big each one is, and how long that array in  WASM memory goes on for.

In `index.js` we set up the initial state of the gameworld and draw it to the canvas before kicking off our renderloop.

```js
drawGrid();
drawCells();
requestAnimationFrame(renderLoop);
```