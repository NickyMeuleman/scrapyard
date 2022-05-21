import * as wasm from "aoc2021";

const fileInputEl = document.querySelector("#file");
const dayInputEl = document.querySelector("#day");
const resultEl = document.querySelector("#result");

function handleFile(event) {
  const file = event.target.files[0];
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

      wasm
        .solve(day, input)
        .then((res) => {
          console.log("res", res);
          resultEl.innerText = res.part1 + "\n" + res.part2;
        })
        .catch((err) => console.log("catch: ", err));
    },
    false
  );

  if (file) {
    reader.readAsText(file);
  }
}

fileInputEl.addEventListener("change", handleFile);
