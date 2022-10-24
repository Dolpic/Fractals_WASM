importScripts("wasm_fractals.js")
const { initialize, compute } = wasm_bindgen;

wasm_promise = wasm_bindgen('./wasm_fractals_bg.wasm')
wasm_promise.then(_ => initialize())

onmessage = function(e){
  const params = e.data
  const byte_per_pixel = 4
  wasm_promise.then(wasm => {
    let compute_fn = compute(
      params.width, 
      params.height, 
      params.pos_x, 
      params.pos_y, 
      params.zoom, 
      params.iterations,
      params.colors,
      params.precise
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
