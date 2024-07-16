import init, { NES } from "../public/pkg"

const keyMap: { [key: string]: number; } = {
    'l': 0,
    'k': 1,
    'Shift': 2,
    'Enter': 3,
    'w': 4,
    's': 5,
    'a': 6,
    'd': 7
};

const SCREEN_WIDTH = 256;
const SCREEN_HEIGHT = 240;
const frame_buffer_length = SCREEN_WIDTH * SCREEN_HEIGHT * 4;

let ctx: AudioContext;
let paused = false;

function scaleCanvas(canvas: HTMLCanvasElement) {
    const scale = Math.min(window.innerWidth / SCREEN_WIDTH, window.innerHeight / SCREEN_HEIGHT, 2);
    canvas.style.width = `${SCREEN_WIDTH * scale}px`;
    canvas.style.height = `${SCREEN_HEIGHT * scale}px`;
}

function setupAudio(nes: NES): AudioContext {
    const context = new AudioContext({ sampleRate: nes.sample_rate() });
    const scriptNode = context.createScriptProcessor(1024 * 2, 0, 1);
    scriptNode.onaudioprocess = (e: AudioProcessingEvent) => {
        if (paused) return;
        nes.load_audio_buffer(e.outputBuffer.getChannelData(0));
    };
    scriptNode.connect(context.destination);
    return context;
}

function setupControls(nes: NES) {
    window.addEventListener('keydown', (e) => handleInput(e, true));
    window.addEventListener('keyup', (e) => handleInput(e, false));

    function handleInput(event: KeyboardEvent, pressed: boolean) {
        if (event.key in keyMap) {
            event.preventDefault();
            nes.update_button(keyMap[event.key], pressed);
        }
    }
}

async function fetchRom(romPath: string): Promise<Uint8Array> {
    const response = await fetch(romPath);
    if (!response.ok) {
        throw new Error(`Failed to fetch ROM: ${response.statusText}`);
    }
    const buffer = await response.arrayBuffer();
    return new Uint8Array(buffer);
}

async function startEmulator(canvas: HTMLCanvasElement) {
    // initialize wasm
    const romData = await fetchRom('roms/mario.nes');
    const wasm = await init();
    const nes = NES.new_nes(romData);

    // setup canvas
    const context = canvas.getContext('2d')!;
    const imageData = context.createImageData(SCREEN_WIDTH, SCREEN_HEIGHT);

    setupControls(nes);
    ctx = setupAudio(nes);
    const startLoop = () => {
        requestAnimationFrame(startLoop);
        if (paused) return;
        nes.step();
        imageData.data.set(new Uint8ClampedArray(wasm.memory.buffer, nes.frame_buffer_pointer(), frame_buffer_length));
        context.putImageData(imageData, 0, 0);
    }

    startLoop();
}

function start() {
    paused = false;
    const canvas = document.querySelector<HTMLCanvasElement>('#screen')!;
    canvas.width = SCREEN_WIDTH;
    canvas.height = SCREEN_HEIGHT;
    scaleCanvas(canvas);
    document.addEventListener('resize', () => scaleCanvas(canvas));

    // start emulator on first click
    document.addEventListener("click", () => {
        if (ctx == undefined) {
            startEmulator(canvas)
        }
    });

    document.getElementById('pause')!.addEventListener('click', () => {
        paused = !paused;
        document.getElementById('pause')!.textContent = paused ? 'Resume' : 'Pause';
    });

    window.addEventListener('blur', function () {
        paused = true;
        document.getElementById('pause')!.textContent = 'Resume';
    });

    window.addEventListener('focus', function () {
        paused = false;
        document.getElementById('pause')!.textContent = 'Pause';
    });
}

document.addEventListener('DOMContentLoaded', start);