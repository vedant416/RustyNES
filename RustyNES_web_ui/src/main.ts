import init, { NES } from "../public/pkg";

document.addEventListener('DOMContentLoaded', start);

function start() {
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

    const canvas = document.querySelector<HTMLCanvasElement>('#screen')!;
    const context = canvas.getContext('2d')!;
    const imageData = context.createImageData(256, 240);
    canvas.width = 256;
    canvas.height = 240;
    const scaleCanvas = () => {
        const scale = Math.min(window.innerWidth / 256, window.innerHeight / 240);
        canvas.style.width = `${256 * scale}px`;
        canvas.style.height = `${240 * scale}px`;
    };
    scaleCanvas();
    window.addEventListener('resize', scaleCanvas);

    (async () => {
        let x = await init();
        const rom = await fetch("roms/mario.nes");
        const bytes = await rom.arrayBuffer();
        const data = new Uint8Array(bytes);
        const nes = NES.new_nes(data);
        const frame_buffer_length = 256 * 240 * 4;
        const frame_buffer_pointer = nes.frame_buffer_pointer();
        window.addEventListener('keydown', (e) => handleInput(e, true));
        window.addEventListener('keyup', (e) => handleInput(e, false));
        render();

        function render() {
            nes.step();
            imageData.data.set(new Uint8ClampedArray(x.memory.buffer, frame_buffer_pointer, frame_buffer_length));
            context.putImageData(imageData, 0, 0);
            requestAnimationFrame(render);
        }

        function handleInput(event: KeyboardEvent, pressed: boolean) {
            if (event.key in keyMap) {
                event.preventDefault();
                nes.update_button(keyMap[event.key], pressed);
            }
        }
    })();
}