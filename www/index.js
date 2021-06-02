import { Universe } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";
import { fps } from "./components";

const CELL_SIZE = 5;
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const universe = Universe.new(256, 256);
const width = universe.width();
const height = universe.height();

/** @type {HTMLCanvasElement} */
const canvas = document.getElementById("game-of-life-canvas");
canvas.width = (CELL_SIZE + 1) * width + 1;
canvas.height = (CELL_SIZE + 1) * height + 1;

const ctx = canvas.getContext("2d");

/**
 * Get the row and the column of the clicked cell on the canvas
 * @param {MouseEvent} event
 * @param {HTMLCanvasElement} canvas
 * @returns {number[]}
 */
const getRowCol = (event, canvas) => {
  const boundingRect = canvas.getBoundingClientRect();

  const scaleX = canvas.width / boundingRect.width;
  const scaleY = canvas.height / boundingRect.height;

  const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
  const canvasTop = (event.clientY - boundingRect.top) * scaleY;

  const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
  const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

  return [row, col];
};

// Spawn a dot, a glider or a pulsar when the canvas is clicked
canvas.addEventListener("click", (event) => {
  const [row, col] = getRowCol(event, canvas);

  if (event.ctrlKey) universe.spawn_glider(row, col);
  else if (event.shiftKey) universe.spawn_pulsar(row, col);
  else universe.toggle_cell(row, col);

  draw();
});

// End of canvas

const playPauseButton = document.getElementById("play-pause");

const play = () => {
  playPauseButton.textContent = "⏸";
  renderLoop();
};

const pause = () => {
  playPauseButton.textContent = "▶";
  cancelAnimationFrame(animationId);
  animationId = null;
};

playPauseButton.addEventListener("click", () =>
  // The animation is paused when its id is null
  animationId === null ? play() : pause()
);

// End of playPauseButton

const slider = document.getElementById("ticks-per-animation-frame");
let ticksPerAnimationFrame = (slider.value = 1);

slider.addEventListener("change", () => {
  ticksPerAnimationFrame = parseInt(slider.value);
});

// End of slider

const randomButton = document.getElementById("random");
randomButton.addEventListener("click", () => {
  universe.randomize();
  draw();
});

const resetButton = document.getElementById("reset");
resetButton.addEventListener("click", () => {
  universe.reset();
  draw();
});

const drawGrid = () => {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;

  // Vertical lines
  for (let i = 0; i <= width; i++) {
    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
  }

  // Horizontal lines
  for (let j = 0; j <= height; j++) {
    ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
    ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
  }

  ctx.stroke();
};

const bitIsSet = (n, arr) => {
  const byte = Math.floor(n / 8);
  const mask = 1 << n % 8;
  return (arr[byte] & mask) === mask;
};

/**
 * Draw every cells that is in a certain state
 * @param {CanvasRenderingContext2D} ctx
 * @param {Uint8Array} cells
 * @param {boolean} deadOrAlive
 * the state of the cells to draw, true will draw alive cells
 */
const drawCellsInState = (ctx, cells, deadOrAlive) => {
  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = row * width + col;
      if (bitIsSet(idx, cells) !== deadOrAlive) continue;

      ctx.fillRect(
        col * (CELL_SIZE + 1) + 1,
        row * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }
};

const drawCells = () => {
  const cellsPtr = universe.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, (width * height) / 8);

  ctx.beginPath();

  ctx.fillStyle = ALIVE_COLOR;
  drawCellsInState(ctx, cells, true);
  // We don't need to draw the dead cells
  // Since they are the same color as the background
  // ctx.fillStyle = DEAD_COLOR;
  // drawCellsInState(ctx, cells, false);

  ctx.stroke();
};

const draw = () => {
  // Clear the canvas
  ctx.fillStyle = DEAD_COLOR;
  ctx.fillRect(0, 0, canvas.width, canvas.height);

  drawGrid();
  drawCells();
};

let animationId = null;

const renderLoop = () => {
  fps.render();

  universe.ticks(ticksPerAnimationFrame);

  draw();

  animationId = requestAnimationFrame(renderLoop);
};

play();
