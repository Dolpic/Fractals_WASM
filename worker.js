importScripts("./wasm_fractals.js")
const { compute } = wasm_bindgen;

init = wasm_bindgen('./wasm_fractals_bg.wasm')

onmessage = function(e){
  const params = e.data
  const byte_per_pixel = 4
  init.then(wasm => {
    let compute_fn = compute(
      params.func,
      params.width, 
      params.height, 
      params.pos_x, 
      params.pos_y, 
      params.zoom, 
      params.iterations,
      params.colors,
      params.precise,
    )
    self.postMessage({
      worker_id:    params.worker_id,
      rendering_id: params.rendering_id,
      chunk_x:      params.chunk_pos_x,
      chunk_y:      params.chunk_pos_y,
      width:        params.width,
      height:       params.height,
      data: new Uint8ClampedArray(wasm.memory.buffer, compute_fn, params.width * params.height * byte_per_pixel)
    })
  })
}
