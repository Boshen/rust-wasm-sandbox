import { game_of_life, tracer } from "wasm-sandbox";

const params = new URLSearchParams(window.location.search)
const canvasDiv = document.getElementById("canvas")
const linksDiv =  document.getElementById("links")

canvasDiv.style.display = 'none'

const run = () => {
  canvasDiv.width = window.innerWidth
  canvasDiv.height = window.innerHeight
  canvasDiv.style.display = 'block'
  linksDiv.style.display = 'none'
}

if (params.has('tracer')) {
  run()
  tracer()
}

if (params.has('life')) {
  run()
  game_of_life()
}
