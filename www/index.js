import { game_of_life, tracer, mendelbrot, threed } from "wasm-sandbox";

const params = new URLSearchParams(window.location.search)
const canvasDiv = document.getElementById("canvas")
const linksDiv =  document.getElementById("links")

linksDiv.style.display = 'none'
canvasDiv.style.display = 'none'

const run = () => {
  canvasDiv.width = window.innerWidth
  canvasDiv.height = window.innerHeight
  canvasDiv.style.display = 'block'
}

if (params.has('tracer')) {
  run()
  tracer()
} else if (params.has('life')) {
  run()
  game_of_life()
} else if (params.has('mendelbrot')) {
  run()
  mendelbrot()
} else if (params.has('3d')) {
  run()
  threed()
} else {
  linksDiv.style.display = 'block'
}
