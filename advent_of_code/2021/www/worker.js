import * as wasm from "aoc2021";

self.onmessage = ({ data }) => {
  console.log("IN worker FROM main", data);
  wasm
    .solve(data.day, data.input)
    .then((res) => {
      const data = {
        part1: res.part1,
        part2: res.part2,
      };
      console.log("IN worker TO main: ", data);
      self.postMessage({ type: "solved", payload: data });
    })
    .catch((err) => {
      self.postMessage({ type: "error", payload: err });
    });
};
