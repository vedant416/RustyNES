import initWasm, { NES } from "../public/pkg/rusty_nes_wasm"
import Stats from "stats.js";
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

async function fetchRom(romPath: string): Promise<Uint8Array> {
    const response = await fetch(romPath);
    if (!response.ok) {
        throw new Error(`Failed to fetch ROM: ${response.statusText}`);
    }
    const buffer = await response.arrayBuffer();
    return new Uint8Array(buffer);
}

let videoContext: number | null = null;
let audioContext: AudioContext | null = null;
let scriptNode: ScriptProcessorNode | null = null;
let isAudioPlaying: boolean = false;
let isVideoPlaying: boolean = false;

let onFrame!: () => void
let onSample!: (e: AudioProcessingEvent) => void
let onLift!: (e: KeyboardEvent) => void
let onPress!: (e: KeyboardEvent) => void
let onRomChange!: (romData: Uint8Array) => void

let nes: NES;
let wasmMemory: WebAssembly.Memory;

let isInit: boolean = false;
let statsAdded = false;
let stats1: Stats;
let stats2: Stats;

const startVideo = () => {
    console.log("startVideo");
    if (!statsAdded) {
        statsAdded = true;

        ////// DEBUG INFO
        let statsDiv = document.querySelector('.stats') as HTMLDivElement;
        stats1 = new Stats();
        stats1.showPanel(0); // FPS
        stats1.dom.style.cssText = '';
        statsDiv.appendChild(stats1.dom);

        stats2 = new Stats();
        stats2.showPanel(1); // MS
        stats2.dom.style.cssText = '';
        statsDiv.appendChild(stats2.dom);
        //////
    }
    if (!isVideoPlaying) {
        isVideoPlaying = true;
        initVideo();
    }
}

const stopVideo = () => {
    if (isVideoPlaying) {
        isVideoPlaying = false;
        if (videoContext) {
            cancelAnimationFrame(videoContext);
            videoContext = null;
        }
    }
}

(window as any).onError = (error: string) => {
    stopBoth();
    isInit = false;
    alert(error);
    console.error(error)
};

const initVideo = () => {
    ////// for throttling loop to 60 fps
    let fps = 60;
    let requiredDelta = Math.floor(1000 / fps);
    let prev = performance.now();
    let delta = 0;

    //// for fps logging
    let totalFrames = 0
    let totalTime = 0;
    let _prev = prev;
    let _delta = 0;
    const loop = (now: number) => {
        /////
        stats1.begin();
        stats2.begin();

        /////
        videoContext = requestAnimationFrame(loop);
        delta = now - prev;
        if (delta < requiredDelta) {
            console.log("Rendering too fast");

            return;
        }
        prev = now - (delta % requiredDelta);
        totalFrames++;
        onFrame();


        ///// Log fps every 60 frames
        _delta = now - _prev;
        totalTime += _delta;
        _prev = now;
        if (totalFrames % 60 === 0) {
            let fps = 1000 / (totalTime / 60);
            console.log(fps.toFixed(2));
            totalFrames = 0;
            totalTime = 0;
        }
        ////

        /////////////////////////////////
        stats1.end();
        stats2.end();

    }
    loop(prev)
}

////////////////
const setupAudio = () => {
    audioContext = new AudioContext({ sampleRate: 44100 });
    audioContext.onstatechange = () => {
        console.log(audioContext?.state);

    };
    scriptNode = audioContext.createScriptProcessor(1024, 0, 1);
    scriptNode.onaudioprocess = (e: AudioProcessingEvent) => {
        onSample(e);
    };
    scriptNode.connect(audioContext.destination);
    console.log("audio setup");
}

const startAudio = async () => {
    if (!audioContext) {
        setupAudio();
    }

    if (isAudioPlaying) {
        return;
    }

    await audioContext?.resume();
    isAudioPlaying = true;
    console.log("audio playing");
}

const stopAudio = async () => {
    if (audioContext && isAudioPlaying) {
        await audioContext.suspend();
        isAudioPlaying = false
        console.log("audio paused");
    }
}

const cleanupAudio = async () => {
    if (audioContext) {
        scriptNode?.disconnect();
        await audioContext.close();
        audioContext = null;
        scriptNode = null;
        isAudioPlaying = false;
        console.log("audio stopped");
    }
}

//////////////////
const startBoth = async () => {
    startVideo();
    if (isMuted) {
        return;
    }
    await startAudio();
}

const stopBoth = async () => {
    await stopAudio();
    stopVideo();
}

const cleanupBoth = async () => {
    stopBoth();
    await cleanupAudio();
}

let playButton = document.getElementById('play')!;
let pauseButton = document.getElementById('pause')!;

let isMuted = false;
let mute = document.getElementById('mute')!;
const setupEventListeners = () => {
    window.addEventListener('keydown', onPress);
    window.addEventListener('keyup', onLift);
    playButton.addEventListener('click', startBoth);
    pauseButton.addEventListener('click', stopBoth);
    mute.addEventListener('click', async () => {
        if (!isVideoPlaying) {
            return;
        }

        if (!audioContext) {
            await startAudio();
        }

        else if (isAudioPlaying) {

            await stopAudio();
        }
        else {
            await startAudio();
        }
        isMuted = !isMuted;
    });
    let liList = document.getElementsByClassName('rom') as HTMLCollectionOf<HTMLLIElement>;
    [...liList].forEach((li) => {
        li.addEventListener('click', async (e) => {
            let url = li.getAttribute('data-name')!;
            changeRom(`roms/${url}.nes`);
        });
    });

    let up = document.getElementById('up')!;
    let down = document.getElementById('down')!;
    let left = document.getElementById('left')!;
    let right = document.getElementById('right')!;
    let a = document.getElementById('a')!;
    let b = document.getElementById('b')!;
    let select = document.getElementById('select')!;
    let start = document.getElementById('start')!;

    const addTouchListeners = (element: HTMLElement, key: number) => {
        element.addEventListener('touchstart', () => {
            element.classList.toggle("pressed");
            navigator.vibrate(70);
            nes.update_button(key, true)
        });
        element.addEventListener('touchend', () => {
            element.classList.toggle("pressed");
            nes.update_button(key, false)
        });
    };

    addTouchListeners(a, 0);
    addTouchListeners(b, 1);
    addTouchListeners(select, 2);
    addTouchListeners(start, 3);
    addTouchListeners(up, 4);
    addTouchListeners(down, 5);
    addTouchListeners(left, 6);
    addTouchListeners(right, 7);
}

const cleanupEventListeners = () => {
    window.removeEventListener('keydown', onPress);
    window.removeEventListener('keyup', onLift);
    playButton.removeEventListener('click', startBoth);
    pauseButton.removeEventListener('click', stopBoth);
}

const changeRom = async (url: string) => {
    await stopBoth();
    const romData = await fetchRom(url);
    onRomChange(romData);
    await startBoth();
}

const init = async (url: string) => {
    isInit = true;
    // setup canvas
    console.log("init called with url ", url);

    const canvas = document.querySelector<HTMLCanvasElement>('#screen')!;
    const context = canvas.getContext('2d')!;
    const imageData = context.createImageData(SCREEN_WIDTH, SCREEN_HEIGHT);
    let parent = document.getElementById('canvas-container')! as HTMLElement;
    console.log(parent);

    const scale = Math.min(window.innerWidth / SCREEN_WIDTH, window.innerHeight / SCREEN_HEIGHT, 3);
    let w = (SCREEN_WIDTH - 10) * scale;
    parent.style.width = w + "px";

    window.addEventListener('resize', () => {
        const scale = Math.min(window.innerWidth / SCREEN_WIDTH, window.innerHeight / SCREEN_HEIGHT, 3);
        let w = (SCREEN_WIDTH - 10) * scale;
        parent.style.width = w + "px";
    });

    // init wasm
    const romData = await fetchRom(url);
    const wasm = await initWasm();
    (window as any).wasm = wasm;
    wasmMemory = wasm.memory;

    // init emulator
    nes = NES.new_nes(romData);
    (window as any).nes = nes;

    // setup controls
    const handleInput = (event: KeyboardEvent, pressed: boolean) => {
        if (event.key in keyMap) {
            event.preventDefault();
            nes.update_button(keyMap[event.key], pressed);
        }
    }

    onPress = (e: KeyboardEvent) => {
        handleInput(e, true);
    }

    onLift = (e: KeyboardEvent) => {
        handleInput(e, false);
    }

    onFrame = () => {
        nes.step();
        imageData.data.set(new Uint8ClampedArray(wasmMemory.buffer, nes.frame_buffer_pointer(), frame_buffer_length));
        context.putImageData(imageData, 0, 0);
    }

    onSample = (e: AudioProcessingEvent) => {
        nes.load_audio_buffer(e.outputBuffer.getChannelData(0));
    }

    onRomChange = (romData: Uint8Array) => {
        nes.change_rom(romData);
    }

    setupEventListeners();
}

function scale(element: HTMLElement) {
    const scale = Math.min(window.innerWidth / SCREEN_WIDTH, window.innerHeight / SCREEN_HEIGHT, 3);
    let w = (SCREEN_WIDTH * scale) - 10;
    element.style.width = w + "px";
}

const main = async () => {
    console.log("inside main");
    let saveButton = document.getElementById('save')!;
    let loadButton = document.getElementById('load')!;
    saveButton.onclick = save;
    loadButton.onclick = load;
    ///
    const canvas = document.getElementById('screen')! as HTMLCanvasElement;
    document.getElementById('downloadBtn')!.addEventListener('click', () => {
        console.log('Download button clicked');
        const scaleFactor = 2;
        const offScreenCanvas = document.createElement('canvas')!;
        const offScreenContext = offScreenCanvas.getContext('2d')!;
        offScreenCanvas.width = canvas.width * scaleFactor;
        offScreenCanvas.height = canvas.height * scaleFactor;
        offScreenCanvas.style.imageRendering = "pixelated";
        offScreenContext.imageSmoothingEnabled = false;
        offScreenContext.drawImage(
            canvas,
            0, 0,
            canvas.width, canvas.height,
            0, 0,
            offScreenCanvas.width, offScreenCanvas.height
        );

        const dataURL = offScreenCanvas.toDataURL('image/png');
        const link = document.createElement('a');
        link.href = dataURL;
        link.download = 'canvas-image.png';
        link.click();
    });
    await init('roms/mario.nes');
    startVideo();
}

let save_buffer: Uint8Array;

const save = () => {
    save_buffer = nes.get_state();
    console.log(save_buffer);
}

const load = () => {
    try {
        nes.set_state(save_buffer);
    } catch (error) {
        console.log(error);
    }
}

document.addEventListener('DOMContentLoaded', main);