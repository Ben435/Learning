const app = () => {
    const canvas = document.getElementById("drawspace");
    const renderButton = document.getElementById("renderButton");

    canvas.setAttribute("width", 640);
    canvas.setAttribute("height", 480);

    renderButton.onclick = render;
}

const render = () => {
    console.log("Stub!");
}

app();
