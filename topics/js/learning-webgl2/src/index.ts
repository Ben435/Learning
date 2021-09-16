import "./style.css"
import * as vertexShaderSource from "./vertex.glsl"
import * as fragmentShaderSource from "./fragment.glsl"

function entry() {
    const canvas = document.getElementById("drawspace") as HTMLCanvasElement

    const gl = canvas.getContext("webgl2")

    if (!gl) {
        throw Error('Webgl2 not supported in this browser!')
    }

    gl.VERTEX_SHADER

    console.log("Initialized!")


    const vertexShader = createShader(gl, gl.VERTEX_SHADER, vertexShaderSource as any)
    const fragmentShader = createShader(gl, gl.FRAGMENT_SHADER, fragmentShaderSource as any)

    const program = createProgram(gl, vertexShader, fragmentShader)

    const positionAttributeLocation = gl.getAttribLocation(program, "a_position")

    const positionBuffer = gl.createBuffer()

    gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer)

    var positions = [
        10, 20,
        80, 20,
        10, 30,
        10, 30,
        80, 20,
        80, 30,
    ];

    gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(positions), gl.STATIC_DRAW)

    const vao = gl.createVertexArray()
    gl.bindVertexArray(vao)
    gl.enableVertexAttribArray(positionAttributeLocation)
    const size = 2
    const type = gl.FLOAT
    const normalize = false
    const stride = 0
    const offset = 0
    gl.vertexAttribPointer(positionAttributeLocation, size, type, normalize, stride, offset)

    requestAnimationFrame(render(canvas, gl, program, vao))
}

function render(canvas: HTMLCanvasElement, gl: WebGL2RenderingContext, program: WebGLProgram, vao: WebGLVertexArrayObject): (now: number) => void {
    const uniformResolution = gl.getUniformLocation(program, 'u_resolution')

    return now => {
        resizeCanvasToDisplaySize(canvas)
        gl.viewport(0, 0, gl.canvas.width, gl.canvas.height);

        gl.clearColor(0,0,0,0)
        gl.clear(gl.COLOR_BUFFER_BIT)
    
        gl.useProgram(program)
        gl.uniform2f(uniformResolution, gl.canvas.width, gl.canvas.height)

        gl.bindVertexArray(vao)
    
        const primitive = gl.TRIANGLES
        const offset = 0
        const count = 6 // <- hax!
        gl.drawArrays(primitive, offset, count)
    }
}

function createShader(gl: WebGL2RenderingContext, type: GLenum, source: string): WebGLShader | null {
    const shader = gl.createShader(type)
    gl.shaderSource(shader, source)
    gl.compileShader(shader)
    const success = gl.getShaderParameter(shader, gl.COMPILE_STATUS)
    if (success) {
        return shader
    }

    console.error(gl.getShaderInfoLog(shader))
    gl.deleteShader(shader)
}

function createProgram(gl: WebGL2RenderingContext, vertexShader: WebGLShader, fragmentShader: WebGLShader): WebGLProgram {
    const program = gl.createProgram()
    gl.attachShader(program, vertexShader)
    gl.attachShader(program, fragmentShader)
    gl.linkProgram(program)
    const success = gl.getProgramParameter(program, gl.LINK_STATUS)
    if (success) {
        return program
    }

    console.error(gl.getProgramInfoLog(program))
    gl.deleteProgram(program)
}

function resizeCanvasToDisplaySize(canvas: HTMLCanvasElement): boolean {
    const displayWidth  = canvas.clientWidth;
    const displayHeight = canvas.clientHeight;

    const needResize = canvas.width  !== displayWidth ||
                        canvas.height !== displayHeight;

    if (needResize) {
        canvas.width  = displayWidth;
        canvas.height = displayHeight;
    }

    return needResize;
}

entry()
