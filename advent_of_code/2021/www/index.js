import * as wasm from "aoc2021";

const fileInputEl = document.querySelector("#file");
const dayInputEl = document.querySelector("#day");
const resultEl = document.querySelector("#result");

function handleFile(event) {
  const file = event.target.files[0];
  console.log(file);

  if (file.type && !file.type.startsWith("text/")) {
    console.log("File is not text.", file.type, file);
    return;
  }

  const reader = new FileReader();

  reader.addEventListener(
    "load",
    () => {
      let input = reader.result;
      const day = Number(dayInputEl.value);

      wasm.solve(day, input).then(({ part1, part2 }) => {
        resultEl.innerText = part1 + "\n" + part2;
      });
    },
    false
  );

  if (file) {
    reader.readAsText(file);
  }
}

fileInputEl.addEventListener("change", handleFile);
