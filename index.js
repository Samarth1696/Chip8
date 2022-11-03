import wasmInit, { Cpu, Keyboard } from "/pkg/Chip8.js";

const CANVAS_WIDTH = 64;
const CANVAS_HEIGHT = 32;

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
function setUpKeyboardListeners(keyboard){
  window.addEventListener("keydown", event => {
  keyboard.key_down(translateKeys[event.code]);
  });

  window.addEventListener("keyup", event => {
  keyboard.key_up(translateKeys[event.code]);
  });
}

const runWasm = async () => {
    
    // Instantiate our wasm module
    await wasmInit("/pkg/Chip8_bg.wasm");

    const keyboard = Keyboard.new();

    const response = await window.fetch(`data/INVADERS`);
    const arraybuffer = await response.arrayBuffer();   
    const game_memory = new Uint8Array(arraybuffer);

    const instanceCpu = Cpu.new(game_memory);

    function initializeCanvas(width, height) {
      const canvas = document.getElementById("canvas");
      const ctx = canvas.getContext("2d");
      ctx.fillStyle = "black";
      ctx.fillRect(0, 0, width, height);

      return ctx;
    }

    setUpKeyboardListeners(keyboard);
    window.setInterval(execute_cycles.bind(this, instanceCpu), 1);

    function execute_cycles(instanceCpu) {
      
      instanceCpu.run_instruction();

      const displayMemory = instanceCpu.get_display_memory();
      const mainCtx = initializeCanvas(CANVAS_WIDTH, CANVAS_HEIGHT);

      updateCanvas(displayMemory, mainCtx, CANVAS_WIDTH, CANVAS_HEIGHT);
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
  };
runWasm();  