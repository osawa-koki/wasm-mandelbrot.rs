import { Space, HueOption } from "wasm-mandelbrot-rs";
import { memory } from "wasm-mandelbrot-rs/wasm_mandelbrot_rs_bg";

const canvas = document.getElementById("canvas");
const ctx = canvas.getContext("2d");

console.log(HueOption.Green);

const space = Space.new();

space.set_params(
  1000,
  1000,
  -2.0,
  1.0,
  -1.5,
  1.5,
  HueOption.Blue,
  30
)

space.compute();

const cellsPtr = space.cells();
const cells = new Uint32Array(memory.buffer, cellsPtr, space.width() * space.height());

const width = space.width();
const height = space.height();

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
