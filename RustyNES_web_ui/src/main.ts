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
let ctx: AudioContext;

function scaleCanvas(canvas: HTMLCanvasElement) {
    const scale = Math.min(window.innerWidth / SCREEN_WIDTH, window.innerHeight / SCREEN_HEIGHT);
    canvas.style.width = `${SCREEN_WIDTH * scale}px`;
    canvas.style.height = `${SCREEN_HEIGHT * scale}px`;
}

function setupAudio(nes: NES): AudioContext {
    const context = new AudioContext({ sampleRate: nes.sample_rate() });
    const scriptNode = context.createScriptProcessor(1024 * 2, 0, 1);
    scriptNode.onaudioprocess = (e: AudioProcessingEvent) => {
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
    const wasm = await init();
    const romData = await fetchRom('roms/mario3.nes');
    const nes = NES.new_nes(romData);
    const frame_buffer_length = SCREEN_WIDTH * SCREEN_HEIGHT * 4;
    const frame_buffer_pointer = nes.frame_buffer_pointer();

    const context = canvas.getContext('2d')!;
    const imageData = context.createImageData(SCREEN_WIDTH, SCREEN_HEIGHT);

    setupControls(nes);
    ctx = setupAudio(nes);
    const startLoop = () => {
        nes.step();
        imageData.data.set(new Uint8ClampedArray(wasm.memory.buffer, frame_buffer_pointer, frame_buffer_length));
        context.putImageData(imageData, 0, 0);
        requestAnimationFrame(startLoop);
    }
    startLoop();
}

function start() {
    const canvas = document.querySelector<HTMLCanvasElement>('#screen')!;
    canvas.width = SCREEN_WIDTH;
    canvas.height = SCREEN_HEIGHT;
    scaleCanvas(canvas);
    document.addEventListener('resize', () => scaleCanvas(canvas));

    document.addEventListener("click", () => {
        if (ctx == undefined) {
            startEmulator(canvas)
        }
    });
}

document.addEventListener('DOMContentLoaded', start);