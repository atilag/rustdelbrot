
export function draw_buffer(){
    let ctx = render_data.ctx
    let image_data = render_data.image_data
    let eight_bit_buffer = render_data.eight_bit_buffer
    image_data.data.set(eight_bit_buffer);
    ctx.putImageData(image_data, 0, 0);
}
