import initWasm, { NES } from "../public/pkg/rusty_nes_wasm";
import Stats from "stats.js";
const keyMap: { [key: string]: number } = {
    l: 0,
    k: 1,
    Shift: 2,
    Enter: 3,
    w: 4,
    s: 5,
    a: 6,
    d: 7,
};

const SCREEN_WIDTH = 256;
const SCREEN_HEIGHT = 240;
const frame_buffer_length = SCREEN_WIDTH * SCREEN_HEIGHT * 4;

let videoContext: number | null = null;
let audioContext: AudioContext | null = null;
let scriptNode: ScriptProcessorNode | null = null;
let isAudioPlaying: boolean = false;
let isVideoPlaying: boolean = false;
let isMuted = true; // default mute
let isInit: boolean = false;

let onFrame!: () => void;
let onSample!: (e: AudioProcessingEvent) => void;
let onLift!: (e: KeyboardEvent) => void;
let onPress!: (e: KeyboardEvent) => void;
let onRomChange!: (romData: Uint8Array) => void;

let nes: NES;
let wasmMemory: WebAssembly.Memory;

let statsAdded = true;
let stats1: Stats;
let stats2: Stats;

///// VIDEO
const setupVideo = () => {
    ////// for throttling loop to 60 fps
    let fps = 60;
    let requiredDelta = Math.floor(1000 / fps);
    let prev = performance.now();
    let delta = 0;

    //// for fps logging
    let totalFrames = 0;
    let totalTime = 0;
    let _prev = prev;
    let _delta = 0;
    const loop = (now: number) => {
        ///// For stats
        // stats1.begin();
        // stats2.begin();

        ///// Throttle to 60 fps
        videoContext = requestAnimationFrame(loop);
        delta = now - prev;
        if (delta < requiredDelta) {
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
            // console.log(fps.toFixed(2));
            totalFrames = 0;
            totalTime = 0;
        }

        ///// For stats
        // stats1.end();
        // stats2.end();
    };
    loop(prev);
};

const startVideo = () => {
    if (!statsAdded) {
        statsAdded = true;

        ////// DEBUG INFO
        let statsDiv = document.querySelector(".stats") as HTMLDivElement;
        stats1 = new Stats();
        stats1.showPanel(0); // FPS
        stats1.dom.style.cssText = "";
        statsDiv.appendChild(stats1.dom);

        stats2 = new Stats();
        stats2.showPanel(1); // MS
        stats2.dom.style.cssText = "";
        statsDiv.appendChild(stats2.dom);
        //////
    }
    if (!isVideoPlaying) {
        isVideoPlaying = true;
        console.log("video started");
        setupVideo();
    }
};

const stopVideo = () => {
    if (isVideoPlaying) {
        isVideoPlaying = false;
        console.log("video stopped");
        if (videoContext) {
            cancelAnimationFrame(videoContext);
            videoContext = null;
        }
    }
};

////// AUDIO
const setupAudio = () => {
    audioContext = new AudioContext({ sampleRate: 44100 });
    scriptNode = audioContext.createScriptProcessor(1024, 0, 1);
    scriptNode.onaudioprocess = (e: AudioProcessingEvent) => {
        onSample(e);
    };
    scriptNode.connect(audioContext.destination);
    console.log("audio setup");
};

const startAudio = async () => {
    if (!audioContext) {
        setupAudio();
    }

    if (!isAudioPlaying) {
        await audioContext?.resume();
        isAudioPlaying = true;
        console.log("audio started");
    }
};

const stopAudio = async () => {
    if (audioContext && isAudioPlaying) {
        await audioContext.suspend();
        isAudioPlaying = false;
        console.log("audio stopped");
    }
};

const cleanupAudio = async () => {
    if (audioContext) {
        scriptNode?.disconnect();
        await audioContext.close();
        audioContext = null;
        scriptNode = null;
        isAudioPlaying = false;
        console.log("audio stopped");
    }
};

const toggleMute = async () => {
    isMuted = !isMuted;
    console.log("muted = ", isMuted);

    if (!isVideoPlaying) {
        return;
    }
    if (isMuted) {
        await stopAudio();
    } else {
        await startAudio();
    }
};

///// START/STOP/DESTROY
const start = async () => {
    startVideo();
    if (!isMuted) {
        await startAudio();
    }
};

const stop = async () => {
    await stopAudio();
    stopVideo();
};

const destroy = async () => {
    stop();
    await cleanupAudio();
    nes.free();
};

////// ERROR HANDLING
(window as any).onError = (error: string) => {
    stop();
    isInit = false;
    alert(error);
    console.error(error);
};

///// SAVE/LOAD/CHANGE ROM
let saveBuffer: Uint8Array;

const saveState = () => {
    saveBuffer = nes.get_state();
};

const loadState = () => {
    try {
        nes.set_state(saveBuffer);
    } catch (error) {
        console.log(error);
    }
};

const fetchRom = async (romPath: string) => {
    const response = await fetch(romPath);
    if (!response.ok) {
        throw new Error(`Failed to fetch ROM: ${response.statusText}`);
    }
    const buffer = await response.arrayBuffer();
    return new Uint8Array(buffer);
};

const changeRom = async (url: string) => {
    await stop();
    const romData = await fetchRom(url);
    onRomChange(romData);
    await start();
};

const downloadImg = () => {
    const canvas = document.getElementById("screen")! as HTMLCanvasElement;
    const scaleFactor = 2;
    const offScreenCanvas = document.createElement("canvas")!;
    const offScreenContext = offScreenCanvas.getContext("2d")!;
    offScreenCanvas.width = canvas.width * scaleFactor;
    offScreenCanvas.height = canvas.height * scaleFactor;
    offScreenCanvas.style.imageRendering = "pixelated";
    offScreenContext.imageSmoothingEnabled = false;
    offScreenContext.drawImage(
        canvas,
        0,
        0,
        canvas.width,
        canvas.height,
        0,
        0,
        offScreenCanvas.width,
        offScreenCanvas.height
    );

    const dataURL = offScreenCanvas.toDataURL("image/png");
    const link = document.createElement("a");
    link.href = dataURL;
    link.download = "canvas-image.png";
    link.click();
};

///// EVENT HANDLING
const setupEventListeners = () => {
    let playButton = document.getElementById("play")!;
    let pauseButton = document.getElementById("pause")!;
    let mute = document.getElementById("mute")!;
    let saveButton = document.getElementById("save")!;
    let loadButton = document.getElementById("load")!;
    let downloadBtn = document.getElementById("downloadBtn")!;

    playButton.onclick = start;
    pauseButton.onclick = stop;
    mute.onclick = toggleMute;
    saveButton.onclick = saveState;
    loadButton.onclick = loadState;
    downloadBtn.onclick = downloadImg;

    let aBtn = document.getElementById("a")!;
    let bBtn = document.getElementById("b")!;
    let selectBtn = document.getElementById("select")!;
    let startBtn = document.getElementById("start")!;
    let upBtn = document.getElementById("up")!;
    let downBtn = document.getElementById("down")!;
    let leftBtn = document.getElementById("left")!;
    let rightBtn = document.getElementById("right")!;
    let btnList = [
        aBtn,
        bBtn,
        selectBtn,
        startBtn,
        upBtn,
        downBtn,
        leftBtn,
        rightBtn,
    ];

    let li = document.getElementsByClassName(
        "rom"
    ) as HTMLCollectionOf<HTMLLIElement>;
    let liList = Array.from(li);

    liList.forEach((li) => {
        li.onclick = async (e) => {
            let url = li.getAttribute("data-name")!;
            changeRom(`roms/${url}.nes`);
        };
    });

    const addTouchListeners = (element: HTMLElement, key: number) => {
        element.ontouchstart = () => {
            element.classList.toggle("pressed");
            navigator.vibrate(70);
            nes.update_button(key, true);
        };

        element.ontouchend = () => {
            element.classList.toggle("pressed");
            nes.update_button(key, false);
        };
    };

    btnList.forEach((btn, index) => {
        addTouchListeners(btn, index);
    });

    window.addEventListener("keydown", onPress);
    window.addEventListener("keyup", onLift);
    window.onblur = () => stop();
    window.onfocus = () => start();

    const cleanupEventListeners = () => {
        window.removeEventListener("keydown", onPress);
        window.removeEventListener("keyup", onLift);
        window.onblur = null;
        window.onfocus = null;
        window.onresize = null;

        playButton.onclick = null;
        pauseButton.onclick = null;
        mute.onclick = null;
        saveButton.onclick = null;
        loadButton.onclick = null;

        liList.forEach((li) => {
            li.onclick = null;
        });

        btnList.forEach((btn) => {
            btn.ontouchstart = null;
            btn.ontouchend = null;
        });
    };

    return cleanupEventListeners;
};

///// INITIALIZATION
const initialize = async (url: string) => {
    // setup canvas
    const canvas = document.querySelector<HTMLCanvasElement>("#screen")!;
    const context = canvas.getContext("2d")!;
    const imageData = context.createImageData(SCREEN_WIDTH, SCREEN_HEIGHT);
    let parent = document.getElementById("canvas-container")! as HTMLElement;

    const scale = Math.min(
        window.innerWidth / SCREEN_WIDTH,
        window.innerHeight / SCREEN_HEIGHT,
        3
    );
    let w = (SCREEN_WIDTH - 10) * scale;
    parent.style.width = w + "px";

    window.onresize = () => {
        const scale = Math.min(
            window.innerWidth / SCREEN_WIDTH,
            window.innerHeight / SCREEN_HEIGHT,
            3
        );
        let w = (SCREEN_WIDTH - 10) * scale;
        parent.style.width = w + "px";
    };

    // init wasm
    const romData = await fetchRom(url);
    const wasm = await initWasm();
    (window as any).wasm = wasm;
    wasmMemory = wasm.memory;

    // init emulator
    nes = NES.new_nes(romData);
    (window as any).nes = nes;
    isInit = true;

    // init controls
    const handleInput = (event: KeyboardEvent, pressed: boolean) => {
        if (event.key in keyMap) {
            event.preventDefault();
            nes.update_button(keyMap[event.key], pressed);
        }
    };

    onPress = (e: KeyboardEvent) => {
        handleInput(e, true);
    };

    onLift = (e: KeyboardEvent) => {
        handleInput(e, false);
    };

    onFrame = () => {
        nes.step();
        imageData.data.set(
            new Uint8ClampedArray(
                wasmMemory.buffer,
                nes.frame_buffer_pointer(),
                frame_buffer_length
            )
        );
        context.putImageData(imageData, 0, 0);
    };

    onSample = (e: AudioProcessingEvent) =>
        nes.load_audio_buffer(e.outputBuffer.getChannelData(0));

    onRomChange = (romData: Uint8Array) => nes.change_rom(romData);

    let cleanup = setupEventListeners();
};

const main = async () => {
    await initialize("roms/mario.nes");
    startVideo();
};

document.addEventListener("DOMContentLoaded", main);
