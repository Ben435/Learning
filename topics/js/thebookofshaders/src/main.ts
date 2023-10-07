import vertexShaderSource from './vertex.vs?raw'
import fragShaderSource from './frag.fs?raw'
import './style.css'

function main() {

  const canvas = document.querySelector<HTMLCanvasElement>('#canvas')!

  const gl = canvas.getContext("webgl2")

  if (!gl) {
    throw Error('`webgl2` not supported in this browser!')
  }

  let shaders: WebGLShader[] = []
  if (vertexShaderSource) {
    shaders.push(createShader(gl, gl.VERTEX_SHADER, vertexShaderSource))
  }
  if (fragShaderSource) {
    shaders.push(createShader(gl, gl.FRAGMENT_SHADER, fragShaderSource))
  }
  const program = createProgram(gl, ...shaders)

  requestAnimationFrame(render(gl, program))
}

function createShader(gl: WebGL2RenderingContext, type: GLenum, source: string): WebGLShader {
  const shader = gl.createShader(type)
  if (!shader) {
    throw `failed to create shader of type: ${type}`
  }
  gl.shaderSource(shader, source)
  gl.compileShader(shader)
  const success = gl.getShaderParameter(shader, gl.COMPILE_STATUS)
  if (success) {
      return shader
  }

  console.error(gl.getShaderInfoLog(shader))
  gl.deleteShader(shader)
  throw `failed to compile shader of type: ${type}`
}

function createProgram(gl: WebGL2RenderingContext, ...shaders: WebGLShader[]): WebGLProgram {
  console.log(shaders)
  const program = gl.createProgram()
  if (!program) {
    throw 'failed to create program of type'
  }
  shaders.forEach(s => gl.attachShader(program, s))
  gl.linkProgram(program)
  const success = gl.getProgramParameter(program, gl.LINK_STATUS)
  if (success) {
      return program
  }

  console.error(gl.getProgramInfoLog(program))
  gl.deleteProgram(program)
  throw `failed to configure program: ${program}`
}

function render(gl: WebGL2RenderingContext, program: WebGLProgram): (time: DOMHighResTimeStamp) => void {
  const uTime = gl.getUniformLocation(program, 'u_time')
  
  return function render(t) {
    gl.viewport(0, 0, gl.canvas.width, gl.canvas.height);

    gl.clearColor(0,0,0,0)
    gl.clear(gl.COLOR_BUFFER_BIT)

    gl.useProgram(program)
    gl.uniform1f(uTime, t)
  }
}

main()
