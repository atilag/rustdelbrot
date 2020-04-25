import * as wasm from "mandelbrot";

const WIDTH = 800;
const HEIGHT = 640;

var offset_x = 0;
var offset_y = 0;

let canvas = document.getElementById('canvas');
let ctx = canvas.getContext("2d");
var image_data = ctx.getImageData(0, 0, WIDTH, HEIGHT);
ctx.font = "20px Georgia";
var buf = new ArrayBuffer(image_data.data.length);
var buf8 = new Uint8ClampedArray(buf);
var data = new Uint32Array(buf);

global.render_data = {
    ctx: ctx,
    image_data: image_data,
    eight_bit_buffer: buf8
}

function resize() {
    let ctx = render_data.ctx;
    ctx.canvas.width = WIDTH;
    ctx.canvas.height = HEIGHT;
    wasm.resize(canvas.width, canvas.height);
}

function render(timestamp){
    let zoom = wasm.update(offset_x, offset_y, data);
    ctx.fillStyle = "#FFFFFF";
    ctx.fillText("zoom: " + zoom, 10, 90);
    requestAnimationFrame(render);
};

// This is not working as expected when we are zooming
window.addEventListener('keydown', e => {
    const OFFSET = 40;
    switch(e.key){
        case "ArrowLeft": offset_x -= OFFSET; break;
        case "ArrowRight": offset_x += OFFSET; break;
        case "ArrowUp": offset_y -= OFFSET; break;
        case "ArrowDown": offset_y += OFFSET; break;
    }
})


resize();
render();
