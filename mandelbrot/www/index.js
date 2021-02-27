'use strict'

function draw(ctx, canvas_w, canvas_h, data) {
  let img = new ImageData(new Uint8ClampedArray(data), canvas_w, canvas_h)
  ctx.putImageData(img, 0, 0)
}
const X_MIN = -1.5;
const X_MAX = 0.5;
const Y_MIN = -1.0;
const Y_MAX = 1.0;
const MAX_ITER = 64;

console.log("start loading wasm");
const mandelbrot = import('../pkg')
  .catch(console.error);

Promise.all([mandelbrot]).then(async function([
  { generate_mandelbrot_set, draw_mandelbrot_set }
]) {
  console.log("finish loading wasm");
  const renderBtn = document.getElementById('render');
  renderBtn.addEventListener('click', () => {
    let jsResult = null;
    let wasmResult = null;
    {
      console.log("wasm only")
      draw_mandelbrot_set();
    }
    {
      console.log("js only")
      const CANVAS_ID = "canvas_js";
      let canvas = document.getElementById(CANVAS_ID);
      let context = canvas.getContext("2d")
      const canvasWidth = canvas.width
      const canvasHeight = canvas.height

      const generateStartTime = Date.now();
      jsResult = logic.genarateMandelbrotSet(canvasWidth, canvasHeight
        , X_MIN, X_MAX, Y_MIN, Y_MAX, MAX_ITER);
      const generateEndTime = Date.now()
      const elasped = generateEndTime - generateStartTime
      console.log(`generate:js\tgenarate_elasped:${elasped}[ms]`)

      const drawStartTime = Date.now();
      draw(context, canvasWidth, canvasHeight, jsResult)
      const drawEndTime = Date.now();
      console.log(`draw:js\tdraw_elasped:${drawEndTime - drawEndTime}[ms]`)
    }
    {
      console.log("wasm + js")
      const CANVAS_ID = "canvas_hybrid";
      let canvas = document.getElementById(CANVAS_ID);
      let context = canvas.getContext("2d");
      const canvasWidth = canvas.width;
      const canvasHeight = canvas.height;
      const generateStartTime = Date.now();
      wasmResult = generate_mandelbrot_set(canvasWidth, canvasHeight,
        X_MIN, X_MAX, Y_MIN, Y_MAX, MAX_ITER);
      const generateEndTime = Date.now();
      const drawStarTime = Date.now();
      draw(context, canvasWidth, canvasHeight, wasmResult);
      const drawEndTime = Date.now();
      const elasped = generateEndTime - generateStartTime;
      console.log(`genereate_elasped:${elasped}`);
      console.log(`draw:${drawEndTime, drawStarTime}`);
    }
    {
      let isSame = true;
      for(let i = 0; i < wasmResult.length; i++){
        if(wasmResult[i] !== jsResult[i]){
          console.log(i, wasmResult[i] ,jsResult[i])
          isSame = false;
          break;
        }
      }
      console.log(`\n(wasmResult === jsResult):${isSame}`)
    }
  })

});



import * as wasm from "hello-wasm-pack";

wasm.greet();

const logic = {
  getNDiverged: function(x0, y0, max_iter) {
    let xn = 0.0;
    let yn = 0.0;
    for (let i = 1; i < max_iter; i++) {
      let x_next = xn * xn - yn * yn + x0;
      let y_next = 2.0 * xn * yn + y0;
      xn = x_next;
      yn = y_next;
      if (xn * xn + yn * yn > 4.0)
        return i

    }
    return max_iter
  },
  genarateMandelbrotSet: function(
    canvas_w,
    canvas_h,
    x_min,
    x_max,
    y_min,
    y_max,
    max_iter,
  ) {
    let data = [];
    for (let i = 0; i < canvas_h; i++) {
      let y = y_min + (y_max - y_min) * i / canvas_h;
      for (let j = 0; j < canvas_w; j++) {
        let x = x_min + (x_max - x_min) * j / canvas_w;
        let iter_index = this.getNDiverged(x, y, max_iter);
        let v = iter_index % 8 * 32
        data.push(v);
        data.push(v);
        data.push(v);
        data.push(255);
      }
    }
    return data;
  }
}
