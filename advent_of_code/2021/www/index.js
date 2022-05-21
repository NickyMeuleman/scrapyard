const fileInputEl = document.querySelector("#file");
const dayInputEl = document.querySelector("#day");
const resultEl = document.querySelector("#result");
const worker = new Worker(new URL("./worker.js", import.meta.url), {
  name: "AoCWorker",
  type: "module",
});
worker.onmessage = function (msg) {
  console.log("IN main FROM worker: ", msg);
  switch (msg.data.type) {
    case "solved": {
      resultEl.innerText =
        msg.data.payload.part1 + "\n" + msg.data.payload.part2;
      return;
    }
    case "error": {
      resultEl.innerText = msg.data.payload.message;
      return;
    }
    default: {
    }
  }
};

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
      const data = { day, input };
      console.log("IN main TO worker: ", data);
      worker.postMessage(data);
    },
    false
  );

  if (file) {
    reader.readAsText(file);
  }
}

fileInputEl.addEventListener("change", handleFile);
