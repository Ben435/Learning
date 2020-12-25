import { mat4 } from 'gl-matrix'
import {} from 'dat.gui'
import { calculateNormals } from './graphics-utils'
import "./style.css"

interface State {
    width: number,
    height: number,
    program: SimpleGLProgram,
    models: Model[],
    modelViewMatrix: mat4,
    projectionMatrix: mat4,
    normalMatrix: mat4,
}

interface SimpleGLProgram extends WebGLProgram {
    aVertexPosition: number,
    aVertexNormal: number,
    uProjectionMatrix: WebGLUniformLocation,
    uModelViewMatrix: WebGLUniformLocation,
    uNormalMatrix: WebGLUniformLocation,
    uLightDirection: WebGLUniformLocation,
    uLightAmbient: WebGLUniformLocation,
    uLightDiffuse: WebGLUniformLocation,
    uMaterialDiffuse: WebGLUniformLocation,
}

interface Model {
    vertexBuffer: WebGLBuffer,
    normalsBuffer: WebGLBuffer,
    indexBuffer: WebGLBuffer,
    vertexArrayObject: WebGLVertexArrayObject,
    indices: number,
}

async function entry(): Promise<void> {
    const canvas = document.getElementById("drawspace") as HTMLCanvasElement

    const gl = canvas.getContext("webgl2")
    gl.clearColor(0.9, 0.9, 0.9, 1)
    gl.clearDepth(100)
    gl.enable(gl.DEPTH_TEST)
    gl.depthFunc(gl.LEQUAL)

    const { width, height } = resize(canvas)

    const fpsTracker = document.createElement('div')
    fpsTracker.innerText = 'N/A'
    fpsTracker.className = 'fps-tracker'
    document.body.appendChild(fpsTracker)

    const frame = (nextState: State): State => {
        const newDimens = resize(canvas)
        const curState = {
            ...nextState,
            ...newDimens
        }
        return draw(curState, gl)
    }

    const vsShader = getShader(gl, "vs")
    const fsShader = getShader(gl, "fs")

    const glProgram = gl.createProgram()
    gl.attachShader(glProgram, vsShader)
    gl.attachShader(glProgram, fsShader)
    gl.linkProgram(glProgram)

    if (!gl.getProgramParameter(glProgram, gl.LINK_STATUS)) {
        console.error('Could not initialize shaders')
    }

    gl.useProgram(glProgram)
    const program = {
        ...glProgram,
        aVertexPosition: gl.getAttribLocation(glProgram, 'aVertexPosition'),
        aVertexNormal: gl.getAttribLocation(glProgram, 'aVertexNormal'),
        uProjectionMatrix: gl.getUniformLocation(glProgram, 'uProjectionMatrix'),
        uModelViewMatrix: gl.getUniformLocation(glProgram, 'uModelViewMatrix'),
        uNormalMatrix: gl.getUniformLocation(glProgram, 'uNormalMatrix'),
        uLightDirection: gl.getUniformLocation(glProgram, 'uLightDirection'),
        uLightAmbient: gl.getUniformLocation(glProgram, 'uLightAmbient'),
        uLightDiffuse: gl.getUniformLocation(glProgram, 'uLightDiffuse'),
        uMaterialDiffuse: gl.getUniformLocation(glProgram, 'uMaterialDiffuse'),
    }

    let models = []

    const vertices = [
        -20, -8, 20, // 0
        -10, -8, 0,  // 1
        10, -8, 0,   // 2
        20, -8, 20,  // 3
        -20, 8, 20,  // 4
        -10, 8, 0,   // 5
        10, 8, 0,    // 6
        20, 8, 20    // 7
      ]
    const indices = [
        0, 5, 4,
        1, 5, 0,
        1, 6, 5,
        2, 6, 1,
        2, 7, 6,
        3, 7, 2
    ]
    const normals = calculateNormals(vertices, indices)
    const model = loadModel(gl, program, vertices, normals, indices)
    models.push(model)

    // for (let i=1; i<=103; i+=1) {
    //     const model = await fetch(`/resources/models/ford-mustang/part${i}.json`)
    //         .then(res => res.json())
    //         .then(data => loadModel(gl, program, data.vertices, data.indices))

    //     models.push(model)
    // }

    initLights(gl, program)

    let state: State = {
        width,
        height,
        program,
        models,
        projectionMatrix: mat4.create(),
        modelViewMatrix: mat4.create(),
        normalMatrix: mat4.create(),
    }

    window.onkeydown = (ev: KeyboardEvent) => {
        if (ev.key === ' ') {
            const startTimeMillis = new Date().getMilliseconds()
            state = frame(state)
            const durationMillis = new Date().getMilliseconds() - startTimeMillis

            console.log(`Rendering in ${durationMillis}ms`)
        }
    }
    console.log('Loaded!')
}

function resize(canvas: HTMLCanvasElement): { width: number, height: number } {
    const { width, height } = canvas

    const windowWidth = window.innerWidth
    const windowHeight = window.innerHeight

    if (width != windowWidth || height != windowHeight) {
        canvas.width = windowWidth
        canvas.height = windowHeight
    }

    return { 
        width: windowWidth,
        height: windowHeight
    }
}

function draw(state: State, gl: WebGL2RenderingContext): State {
    const {
        width,
        height,
        models,
        projectionMatrix,
        modelViewMatrix,
        normalMatrix,
        program,
    } = state
    gl.clearColor(1.0, 1.0, 1.0, 1.0)
    gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT)
    gl.viewport(0, 0, state.width, state.height)

    mat4.perspective(projectionMatrix, 45, width / height, 0.1, 10000)
    mat4.identity(modelViewMatrix)
    mat4.translate(modelViewMatrix, modelViewMatrix, [0, 0, -40])

    mat4.copy(normalMatrix, modelViewMatrix)
    mat4.invert(normalMatrix, normalMatrix)
    mat4.transpose(normalMatrix, normalMatrix)

    gl.uniformMatrix4fv(program.uModelViewMatrix, false, modelViewMatrix)
    gl.uniformMatrix4fv(program.uProjectionMatrix, false, projectionMatrix)
    gl.uniformMatrix4fv(program.uNormalMatrix, false, normalMatrix)

    models.forEach(model => {
        gl.bindVertexArray(model.vertexArrayObject)
        gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, model.indexBuffer)

        gl.drawElements(gl.TRIANGLES, model.indices, gl.UNSIGNED_SHORT, 0)
    })

    // Clean
    gl.bindVertexArray(null)
    gl.bindBuffer(gl.ARRAY_BUFFER, null)
    gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, null)

    return state
}

function getShader(gl: WebGL2RenderingContext, id: string) {
    const script = document.getElementById(id) as HTMLScriptElement
    const shaderString = script.text.trim()

    // Assign shader depending on the type of shader
    let shader
    if (script.type === 'x-shader/x-vertex') {
      shader = gl.createShader(gl.VERTEX_SHADER)
    }
    else if (script.type === 'x-shader/x-fragment') {
      shader = gl.createShader(gl.FRAGMENT_SHADER)
    }
    else {
        console.error(`Failed to match shader with type: '${script.type}'`)
      return null
    }

    // Compile the shader using the supplied shader code
    gl.shaderSource(shader, shaderString)
    gl.compileShader(shader)

    // Ensure the shader is valid
    if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
      console.error(gl.getShaderInfoLog(shader))
      return null
    }

    return shader
  }

function loadModel(gl: WebGL2RenderingContext, program: SimpleGLProgram, vertices: number[], normals: number[], indices: number[]): Model {
    const vertexArrayObject = gl.createVertexArray()
    gl.bindVertexArray(vertexArrayObject)

    const vertexBuffer = gl.createBuffer()
    gl.bindBuffer(gl.ARRAY_BUFFER, vertexBuffer)
    gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(vertices), gl.STATIC_DRAW)

    gl.vertexAttribPointer(program.aVertexPosition, 3, gl.FLOAT, false, 0, 0)
    gl.enableVertexAttribArray(program.aVertexPosition)

    const normalsBuffer = gl.createBuffer()
    gl.bindBuffer(gl.ARRAY_BUFFER, normalsBuffer)
    gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(normals), gl.STATIC_DRAW)

    gl.enableVertexAttribArray(program.aVertexNormal)
    gl.vertexAttribPointer(program.aVertexNormal, 3, gl.FLOAT, false, 0, 0)

    const indexBuffer = gl.createBuffer()
    gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, indexBuffer)
    gl.bufferData(gl.ELEMENT_ARRAY_BUFFER, new Uint16Array(indices), gl.STATIC_DRAW)

    gl.bindVertexArray(null)
    gl.bindBuffer(gl.ARRAY_BUFFER, null)
    gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, null)

    return {
        vertexBuffer,
        normalsBuffer,
        indexBuffer,
        vertexArrayObject,
        indices: indices.length,
    }
}

function initLights(gl: WebGL2RenderingContext, program: SimpleGLProgram) {
    gl.uniform3fv(program.uLightDirection, [0, 0, -1])
    gl.uniform4fv(program.uLightAmbient, [0.01, 0.01, 0.01, 1])
    gl.uniform4fv(program.uLightDiffuse, [0.5, 0.5, 0.5, 1])
    gl.uniform4f(program.uMaterialDiffuse, 0.1, 0.5, 0.8, 1)
}


entry()
