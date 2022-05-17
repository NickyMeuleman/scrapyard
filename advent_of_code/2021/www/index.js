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
      let result = wasm.solve(day, input);
      resultEl.innerText = result.part1 + "\n" + result.part2;
      console.log(result);
    },
    false
  );

  if (file) {
    reader.readAsText(file);
  }
}


fileInputEl.addEventListener("change", handleFile);
