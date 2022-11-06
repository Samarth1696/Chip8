import wasmInit, { Cpu } from "/pkg/Chip8.js";

const CANVAS_WIDTH = 64;
const CANVAS_HEIGHT = 32;

const ROMS = [
  "15PUZZLE",
  "BLINKY",
  "BLITZ",
  "BRIX",
  "CONNECT4",
  "GUESS",
  "HIDDEN",
  "IBM",
  "KALEID",
  "MAZE",
  "MERLIN",
  "MISSILE",
  "PONG",
  "PONG2",
  "PUZZLE",
  "SYZYGY",
  "TANK",
  "TETRIS",
  "TICTAC",
  "UFO",
  "VBRIX",
  "VERS",
  "WIPEOFF",
  "INVADERS"
];

const translateKeys = {
  Digit1: 0x1, // 1
  Digit2: 0x2, // 2
  Digit3: 0x3, // 3
  Digit4: 0xc, // 4

  KeyQ: 0x4, // Q
  KeyW: 0x5, // W
  KeyE: 0x6, // E
  KeyR: 0xd, // R

  KeyA: 0x7, // A
  KeyS: 0x8, // S
  KeyD: 0x9, // D
  KeyF: 0xE, // F

  KeyZ: 0xA, // Z
  KeyX: 0x0, // X
  KeyC: 0xB, // C
  KeyV: 0xF, // V
};

const runWasm = async () => {

  // Instantiate our wasm module
  await wasmInit("/pkg/Chip8_bg.wasm");

  async function initializeRom(rom) {
    const response = await window.fetch(`data/${rom}`);
    const arraybuffer = await response.arrayBuffer();
    const game_memory = new Uint8Array(arraybuffer);

    return Cpu.new(game_memory);
  }

  ROMS.forEach(rom => {
    $("#roms").append(`<option value='${rom}'>${rom}</option>`);
  });
  $("#roms")[0].value = 'INVADERS';
  var instanceCpu = await initializeRom($("#roms")[0].value);

  document.getElementById("roms").addEventListener("change", async e => {
    instanceCpu = await initializeRom(e.target.value);
  });

  document.getElementById("reset").addEventListener("click", async e => {
    instanceCpu = await initializeRom($("#roms")[0].value);
  });

  function initializeCanvas(width, height) {
    const canvas = document.getElementById("canvas");
    const ctx = canvas.getContext("2d");
    ctx.fillStyle = "black";
    ctx.fillRect(0, 0, width, height);

    return ctx;
  }

  let running = false;
  const runloop = () => {
    if (running) {
      for (var i = 0; i < 9; i++) {
        execute_cycle(instanceCpu);
      }
      instanceCpu.tick();
    }
    const displayMemory = instanceCpu.get_display_memory();
    const mainCtx = initializeCanvas(CANVAS_WIDTH, CANVAS_HEIGHT);
    updateCanvas(displayMemory, mainCtx, CANVAS_WIDTH, CANVAS_HEIGHT);

    window.requestAnimationFrame(runloop);
  };
  window.requestAnimationFrame(runloop);

  const runButton = document.getElementById("run");
  runButton.addEventListener("click", () => {
    if (running) {
      running = false;
      runButton.innerHTML = "Start";
    } else {
      running = true;
      runButton.innerHTML = "Stop";
    }
  });

  function execute_cycle(instanceCpu) {
    instanceCpu.run_instruction();
  }

  function updateCanvas(displayState, ctx, width, height) {
    const imageData = ctx.createImageData(width, height);
    for (let i = 0; i < displayState.length; i++) {
      imageData.data[i * 4] = displayState[i] === 1 ? 0xff : 0;
      imageData.data[i * 4 + 1] = displayState[i] === 1 ? 0xff : 0;
      imageData.data[i * 4 + 2] = displayState[i] === 1 ? 0xff : 0;
      imageData.data[i * 4 + 3] = 255;
    }

    ctx.putImageData(imageData, 0, 0);
  }

  window.addEventListener("keyup", event => {
    instanceCpu.key_up(translateKeys[event.code]);
  });

  window.addEventListener("keydown", event => {
    instanceCpu.key_down(translateKeys[event.code]);
  });


};

runWasm();  
