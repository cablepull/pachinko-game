import init, { Game } from "./pkg/pachinko_game.js";

const canvas = document.getElementById("game");
const ctx = canvas.getContext("2d");
const dropBtn = document.getElementById("dropBtn");
const autoBtn = document.getElementById("autoBtn");
const resetBtn = document.getElementById("resetBtn");
const scoreEl = document.getElementById("score");
const ballsEl = document.getElementById("balls");
const binsEl = document.getElementById("bins");

let game;
let auto = false;
let autoAccumulator = 0;

function drawCircle(x, y, r, fill) {
  ctx.beginPath();
  ctx.arc(x, y, r, 0, Math.PI * 2);
  ctx.fillStyle = fill;
  ctx.fill();
}

function drawBoard() {
  const width = game.width();
  const height = game.height();

  ctx.clearRect(0, 0, width, height);

  const pegs = game.pegs_flat();
  for (let i = 0; i < pegs.length; i += 3) {
    drawCircle(pegs[i], pegs[i + 1], pegs[i + 2], "#88a7ff");
  }

  const bins = game.bin_count();
  const binW = width / bins;
  const floorY = height - 30;

  ctx.strokeStyle = "#5d74b3";
  ctx.lineWidth = 2;
  ctx.beginPath();
  ctx.moveTo(0, floorY);
  ctx.lineTo(width, floorY);
  ctx.stroke();

  for (let i = 1; i < bins; i++) {
    const x = i * binW;
    ctx.beginPath();
    ctx.moveTo(x, floorY);
    ctx.lineTo(x, height);
    ctx.stroke();
  }

  const balls = game.balls_flat();
  for (let i = 0; i < balls.length; i += 3) {
    drawCircle(balls[i], balls[i + 1], balls[i + 2], "#ffd166");
  }
}

function updateHud() {
  scoreEl.textContent = `Score: ${game.score()}`;
  ballsEl.textContent = `Balls: ${game.ball_count()}`;

  const hits = game.bins();
  const points = game.bin_scores();
  binsEl.innerHTML = "";
  hits.forEach((count, i) => {
    const node = document.createElement("div");
    node.className = "bin";
    node.textContent = `Bin ${i + 1}: ${count} (${points[i]} pts)`;
    binsEl.appendChild(node);
  });
}

let last = 0;
function frame(ts) {
  if (!last) last = ts;
  const dt = Math.min((ts - last) / 1000, 0.033);
  last = ts;

  if (auto) {
    autoAccumulator += dt;
    if (autoAccumulator > 0.35) {
      autoAccumulator = 0;
      game.auto_drop_center();
    }
  }

  game.update(dt);
  drawBoard();
  updateHud();
  requestAnimationFrame(frame);
}

function dropAtClientX(clientX) {
  const rect = canvas.getBoundingClientRect();
  const x = ((clientX - rect.left) / rect.width) * canvas.width;
  game.drop_ball(x);
}

async function start() {
  await init();
  game = new Game(canvas.width, canvas.height, 9);

  dropBtn.addEventListener("click", () => game.drop_ball(canvas.width * 0.5));
  resetBtn.addEventListener("click", () => game.reset());
  autoBtn.addEventListener("click", () => {
    auto = !auto;
    autoBtn.textContent = `Auto: ${auto ? "On" : "Off"}`;
    autoBtn.setAttribute("aria-pressed", auto ? "true" : "false");
  });

  canvas.addEventListener("click", (e) => dropAtClientX(e.clientX));

  requestAnimationFrame(frame);
}

start();
