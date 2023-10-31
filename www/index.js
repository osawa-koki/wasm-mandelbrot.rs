import { Space, HueOption } from "wasm-mandelbrot-rs";
import { memory } from "wasm-mandelbrot-rs/wasm_mandelbrot_rs_bg";

const widthElement = document.getElementById("width");
const heightElement = document.getElementById("height");
const xMinElement = document.getElementById("x_min");
const xMaxElement = document.getElementById("x_max");
const yMinElement = document.getElementById("y_min");
const yMaxElement = document.getElementById("y_max");
const hueOptionElement = document.getElementById("hue_option");
const maxIterationsElement = document.getElementById("max_iterations");
const buttonElement = document.getElementById("button");

widthElement.value = 1000;
heightElement.value = 1000;
xMinElement.value = -2.0;
xMaxElement.value = 1.0;
yMinElement.value = -1.5;
yMaxElement.value = 1.5;
hueOptionElement.value = "red";
maxIterationsElement.value = 30;

const canvas = document.getElementById("canvas");
const ctx = canvas.getContext("2d");

const space = Space.new();

function draw() {
  const width = parseInt(widthElement.value);
  const height = parseInt(heightElement.value);
  const xMin = parseFloat(xMinElement.value);
  const xMax = parseFloat(xMaxElement.value);
  const yMin = parseFloat(yMinElement.value);
  const yMax = parseFloat(yMaxElement.value);
  const hueOption = (() => {
    switch (hueOptionElement.value) {
      case "red":
        return HueOption.Red
      case "green":
        return HueOption.Green
      case "blue":
        return HueOption.Blue
    }
  })();
  const maxIterations = parseInt(maxIterationsElement.value);

  space.set_params(
    width,
    height,
    xMin,
    xMax,
    yMin,
    yMax,
    hueOption,
    maxIterations,
  )

  const cellsPtr = space.cells();
  const cells = new Uint32Array(memory.buffer, cellsPtr, space.width() * space.height());

  space.compute();

  canvas.width = width
  canvas.height = height

  cells.forEach((cell, i) => {
    const r = (cell >> 16) & 0xff;
    const g = (cell >> 8) & 0xff;
    const b = (cell >> 0) & 0xff;
    const a = 255;
    const x = i % width;
    const y = Math.floor(i / width);
    ctx.fillStyle = `rgba(${r}, ${g}, ${b}, ${a})`;
    ctx.fillRect(x, y, 1, 1);
  });
}

buttonElement.addEventListener("click", draw);
