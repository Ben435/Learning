import "./style.css"

interface State {
    width: number,
    height: number,
    squareVertexBuffer: WebGLBuffer,
    squareIndexBuffer: WebGLBuffer,
    squareVAO: WebGLVertexArrayObject,
    program: SimpleGLProgram,
}

interface SimpleGLProgram extends WebGLProgram {
    aVertexPosition: number
}

function entry() {
    const canvas = document.getElementById("drawspace") as HTMLCanvasElement

    const gl = canvas.getContext("webgl2")

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

    const glProgram = gl.createProgram();
    gl.attachShader(glProgram, vsShader);
    gl.attachShader(glProgram, fsShader);
    gl.linkProgram(glProgram);

    if (!gl.getProgramParameter(glProgram, gl.LINK_STATUS)) {
        console.error('Could not initialize shaders');
    }

    gl.useProgram(glProgram);
    const program = {
        ...glProgram,
        aVertexPosition: gl.getAttribLocation(glProgram, 'aVertexPosition')
    }

    const bufferData = initBuffers(gl, program)

    let state: State = {
        width,
        height,
        program,
        ...bufferData,
    }

    window.onkeydown = (ev: KeyboardEvent) => {
        if (ev.key === ' ') {
            state = frame(state)
        }
    }
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
        squareVertexBuffer,
        squareIndexBuffer,
        squareVAO,
    } = state
    gl.clearColor(1.0, 1.0, 1.0, 1.0);
    gl.clear(gl.COLOR_BUFFER_BIT);
    gl.viewport(0, 0, state.width, state.height);

    // Use the buffers we've constructed
    gl.bindBuffer(gl.ARRAY_BUFFER, squareVertexBuffer);

    // Bind IBO
    gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, squareIndexBuffer);

    gl.bindVertexArray(squareVAO)

    // Draw to the scene using triangle primitives
    gl.drawElements(gl.TRIANGLES, 6, gl.UNSIGNED_SHORT, 0);

    // Clean
    gl.bindVertexArray(null)
    gl.bindBuffer(gl.ARRAY_BUFFER, null);
    gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, null);

    return {
        ...state
    }
}

function getShader(gl: WebGL2RenderingContext, id: string) {
    const script = document.getElementById(id) as HTMLScriptElement;
    const shaderString = script.text.trim();

    // Assign shader depending on the type of shader
    let shader;
    if (script.type === 'x-shader/x-vertex') {
      shader = gl.createShader(gl.VERTEX_SHADER);
    }
    else if (script.type === 'x-shader/x-fragment') {
      shader = gl.createShader(gl.FRAGMENT_SHADER);
    }
    else {
        console.error(`Failed to match shader with type: '${script.type}'`)
      return null;
    }

    // Compile the shader using the supplied shader code
    gl.shaderSource(shader, shaderString);
    gl.compileShader(shader);

    // Ensure the shader is valid
    if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
      console.error(gl.getShaderInfoLog(shader));
      return null;
    }

    return shader;
  }

function initBuffers(gl: WebGL2RenderingContext, program: SimpleGLProgram) {
    const vertices = [
        -0.5, 0.5, 0,
        -0.5, -0.5, 0,
        0.5, -0.5, 0,
        0.5, 0.5, 0
    ]

    const indices = [0, 1, 2, 0, 2, 3]

    const squareVAO = gl.createVertexArray()
    gl.bindVertexArray(squareVAO);

    const squareVertexBuffer = gl.createBuffer()
    gl.bindBuffer(gl.ARRAY_BUFFER, squareVertexBuffer)
    gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(vertices), gl.STATIC_DRAW)

    gl.vertexAttribPointer(program.aVertexPosition, 3, gl.FLOAT, false, 0, 0);
    gl.enableVertexAttribArray(program.aVertexPosition);

    const squareIndexBuffer = gl.createBuffer()
    gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, squareIndexBuffer)
    gl.bufferData(gl.ELEMENT_ARRAY_BUFFER, new Uint16Array(indices), gl.STATIC_DRAW)

    gl.bindVertexArray(null)
    gl.bindBuffer(gl.ARRAY_BUFFER, null)
    gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, null)

    return {
        squareVertexBuffer,
        squareIndexBuffer,
        squareVAO,
    }
}


entry()
