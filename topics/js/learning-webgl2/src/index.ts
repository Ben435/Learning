import "./style.css"

interface State {
    width: number,
    height: number,
    program: SimpleGLProgram,
    models: Model[],
}

interface SimpleGLProgram extends WebGLProgram {
    aVertexPosition: number
}

interface Model {
    vertexBuffer: WebGLBuffer,
    indexBuffer: WebGLBuffer,
    vertexArrayObject: WebGLVertexArrayObject,
}

async function entry(): Promise<void> {
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

    let models = []
    for (let i=1; i<=103; i+=1) {
        const model = await fetch(`/resources/models/ford-mustang/part${i}.json`)
            .then(res => res.json())
            .then(data => loadModel(gl, program, data.vertices, data.indices))

        models.push(model)
    }

    let state: State = {
        width,
        height,
        program,
        models,
    }

    window.onkeydown =    (ev: KeyboardEvent) => {
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
        models,
    } = state
    gl.clearColor(1.0, 1.0, 1.0, 1.0);
    gl.clear(gl.COLOR_BUFFER_BIT);
    gl.viewport(0, 0, state.width, state.height);

    models.forEach(model => {
        gl.bindVertexArray(model.vertexArrayObject)
        gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, model.indexBuffer);

        gl.drawElements(gl.TRIANGLES, 6, gl.UNSIGNED_SHORT, 0);
    })

    // Clean
    gl.bindVertexArray(null)
    gl.bindBuffer(gl.ARRAY_BUFFER, null);
    gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, null);

    return state
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

function loadModel(gl: WebGL2RenderingContext, program: SimpleGLProgram, vertices: number[], indices: number[]): Model {
    const vertexArrayObject = gl.createVertexArray()
    gl.bindVertexArray(vertexArrayObject);

    const vertexBuffer = gl.createBuffer()
    gl.bindBuffer(gl.ARRAY_BUFFER, vertexBuffer)
    gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(vertices), gl.STATIC_DRAW)

    gl.vertexAttribPointer(program.aVertexPosition, 3, gl.FLOAT, false, 0, 0);
    gl.enableVertexAttribArray(program.aVertexPosition);

    const indexBuffer = gl.createBuffer()
    gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, indexBuffer)
    gl.bufferData(gl.ELEMENT_ARRAY_BUFFER, new Uint16Array(indices), gl.STATIC_DRAW)

    gl.bindVertexArray(null)
    gl.bindBuffer(gl.ARRAY_BUFFER, null)
    gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, null)

    return {
        vertexBuffer,
        indexBuffer,
        vertexArrayObject,
    }
}


entry()
