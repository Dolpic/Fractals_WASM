class Fractal{
    constructor(canvas, nb_workers=15){
      this.canvas = canvas
      this.ctx = this.canvas.getContext("2d")
      
      this.current_chunk = 0
      this.current_rendering_id = 0

      this.set_parameters()
      this.set_colors()

      this.workers = []
      for(let i=0; i<nb_workers; i++){
        this.create_worker()
      }
    }

    set_colors(color_in="#ffffff", color_out="#000000", color_highlight="#ffffff"){
      let inputs = [color_in, color_out, color_highlight]
      this.colors = []

      inputs.forEach(c =>{
        let val = parseInt(c.substring(1), 16);
        this.colors.push((val >> 16) & 255)
        this.colors.push((val >> 8) & 255)
        this.colors.push(val & 255)
      })
    }

    set_parameters(func=1, dimensions=[800,800], position=[0, 0], zoom=200, iterations=300, chunk_size=100, precise=false){
      this.func       = func
      this.img_width  = dimensions[0]
      this.img_height = dimensions[1]
      this.img_pos_x  = position[0]
      this.img_pos_y  = position[1]
      this.img_zoom   = parseInt(zoom).toString()
      this.iterations = iterations
      this.precise    = precise

      if(this.canvas.width != this.img_width || this.canvas.height != this.img_height || chunk_size != this.chunk_size){
        this.chunk_size    = chunk_size
        this.canvas.width  = this.img_width
        this.canvas.height = this.img_height

        this.nb_chunk_width  = Math.ceil(this.img_width /this.chunk_size)
        this.nb_chunk_height = Math.ceil(this.img_height/this.chunk_size)
        this.nb_chunks       = this.nb_chunk_width*this.nb_chunk_height
        this.border_chunk_height = (1-this.nb_chunk_height)*this.chunk_size + this.img_height
        this.border_chunk_width  = (1-this.nb_chunk_width) *this.chunk_size + this.img_width
      }
    }

    get_next_chunk(){
      if(this.current_chunk > this.nb_chunks){return null}

      const chunk_x = (this.current_chunk%this.nb_chunk_width)
      const chunk_y = Math.floor(this.current_chunk/this.nb_chunk_width)

      const chunk_width  = chunk_x == this.nb_chunk_width-1  ? this.border_chunk_width  : this.chunk_size
      const chunk_height = chunk_y == this.nb_chunk_height-1 ? this.border_chunk_height : this.chunk_size

      const pos_x = (chunk_x*this.chunk_size + (chunk_width-this.img_width)/2)   / this.img_zoom
      const pos_y = (chunk_y*this.chunk_size + (chunk_height-this.img_height)/2) / this.img_zoom

      const result = {
        func:    this.func,
        width:       chunk_width,
        height:      chunk_height,
        zoom:        this.img_zoom,
        pos_x:       pos_x + this.img_pos_x,
        pos_y:       pos_y + this.img_pos_y,
        chunk_pos_x: chunk_x*this.chunk_size,
        chunk_pos_y: chunk_y*this.chunk_size,
        iterations:  this.iterations,
        colors:      this.colors,
        precise:     this.precise
      }

      this.current_chunk++
      return result
    }

    start_rendering(){
      this.current_chunk = 0
      this.current_rendering_id++
      for(let i=0; i<this.workers.length; i++) {
        let next_chunk = this.get_next_chunk()
        if(next_chunk != null){
          next_chunk.worker_id    = i
          next_chunk.rendering_id = this.current_rendering_id
          this.workers[i].postMessage(next_chunk)
        }
      }
    }

    create_worker(){
      this.workers.push(new Worker("worker.js"))
      this.workers.at(-1).addEventListener("message", e => {
        let p = e.data
        let cur_worker = this.workers[p.worker_id]
        if(p.rendering_id == this.current_rendering_id){
          this.ctx.putImageData(new ImageData(p.data, p.width, p.height), p.chunk_x, p.chunk_y);
          let next_chunk = this.get_next_chunk()
          if(next_chunk != null){
            next_chunk.worker_id    = p.worker_id
            next_chunk.rendering_id = p.rendering_id
            cur_worker.postMessage(next_chunk)
          }
        }
      })
    }
  }