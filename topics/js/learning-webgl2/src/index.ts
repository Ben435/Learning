import "./style.css"

function entry() {
    const canvas = document.getElementById("drawspace") as HTMLCanvasElement

    const gl = canvas.getContext("webgl2")

    console.log("Initialized!")
}

entry()
