import init, { solve } from "../pkg/aoc2015.js";
console.log(
  "I'm at the Pizza Hut. I'm at the Taco Bell. I'm at the combination Pizza Hut and Taco Bell"
);

// It may look like this: `await init('./pkg/without_a_bundler_bg.wasm');`,
// but there is also a handy default inside `init` function, which uses
// `import.meta` to locate the wasm file relatively to js file.
await init();

self.onmessage = async ({ data }) => {
  console.log("IN worker FROM main", data);
  try {
    const result = await solve(data.day, data.input);
    console.log("IN worker TO main: ", result);
    self.postMessage({
      type: "solved",
      payload: { part1: result.part1, part2: result.part2 },
    });
  } catch (err) {
    self.postMessage({ type: "error", payload: err });
  }
};
