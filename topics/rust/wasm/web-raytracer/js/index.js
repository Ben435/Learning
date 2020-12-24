import * as util from './utility'

async function main() {
    const wasm = await import("../pkg/index.js").catch(console.error);

    const canvas = document.getElementById("drawspace");

    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
    
    const gl = canvas.getContext("webgl2", { antialias: false });

    const program = util.createProgram(gl, util.getShaderSource("vs"), util.getShaderSource("fs"));
    const positionAttrLoc = gl.getAttribLocation(program, "a_position");
    const resolutionUniformLoc = gl.getUniformLocation(program, "u_resolution");

    const positionBuffer = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);

    const positions = [
        10, 20,
        80, 20,
        10, 30,
        10, 30,
        80, 20,
        80, 30,
    ];

    gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(positions), gl.STATIC_DRAW);

    const vao = gl.createVertexArray();
    gl.bindVertexArray(vao)

    gl.enableVertexAttribArray(positionAttrLoc)
    gl.vertexAttribPointer(
        positionAttrLoc,
        2,
        gl.FLOAT,
        false,
        0,
        0
    )

    gl.bindVertexArray(null)

    const draw = timeElapsed => {
        resize(canvas);
        gl.viewport(0, 0, canvas.width, canvas.height);

        gl.clearColor(0, 0, 0, 0);
        gl.clear(gl.COLOR_BUFFER_BIT);
        gl.useProgram(program);
        gl.uniform2f(resolutionUniformLoc, canvas.width, canvas.height);

        gl.bindVertexArray(vao);
        gl.drawArrays(gl.TRIANGLES, 0, 6);

        window.requestAnimationFrame(draw);
    }

    window.requestAnimationFrame(draw);
}

function resize(canvas) {
    // Lookup the size the browser is displaying the canvas.
    const displayWidth  = canvas.clientWidth;
    const displayHeight = canvas.clientHeight;
  
    // Check if the canvas is not the same size.
    if (canvas.width  !== displayWidth ||
        canvas.height !== displayHeight) {
  
      // Make the canvas the same size
      canvas.width  = displayWidth;
      canvas.height = displayHeight;
    }
  }

main()
