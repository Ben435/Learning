import Vec3 from './vec3';
import scene from './scene';
import trace from './trace';
import { width, height, MODE } from './constants';

const app = () => {
    const canvas = document.getElementById("drawspace");
    const renderButton = document.getElementById("renderButton");

    canvas.setAttribute("width", width);
    canvas.setAttribute("height", height);

    const context = canvas.getContext("2d");

    const renderModeSelector = document.getElementById("renderMode");
    Object.entries(MODE).reverse().forEach(([label, val]) => {
        const opt = document.createElement("option");
        opt.innerText = label[0].toUpperCase() + label.slice(1).toLowerCase();
        opt.setAttribute("value", val)

        renderModeSelector.appendChild(opt);
    });

    renderButton.onclick = async() => {
        renderButton.setAttribute("disabled", true);

        const options = {
            mode: renderModeSelector.value,
        };

        await render(context, options);

        renderButton.removeAttribute("disabled");
    }
}

const render = async (drawContext, options={}) => {
    const image = drawContext.createImageData(width, height);
    const geometries = scene;

    console.log('Options: ', options);

    const invWidth = 1 / width;
    const invHeight = 1 / height;
    const fov = 30;
    const aspectRatio = width / height;
    const angle = Math.tan(Math.PI * 0.5 * fov / 180);

    const startTime = performance.now();

    for (let y=0; y<height; y++) {
        for (let x=0; x<width; x++) {
            const rayOrigin = new Vec3();
            const xx = (2 * ((x + 0.5) * invWidth) - 1) * angle * aspectRatio;
            const yy = (1 - 2 * ((y + 0.5) * invHeight)) * angle;
            const rayDirection = new Vec3(xx, yy, -1).normalize();

            const pixel = trace(rayOrigin, rayDirection, geometries, 0, options);

            const index = (y * width + x) * 4;
            
            try {
                image.data.set(pixel.toColor(), index);
            } catch (e) {
                console.error('Catch toColor convert', x, y, e);
                break;
            }
        }
    }
    const endTime = performance.now();

    const duration = endTime - startTime;

    console.log(`Took ${duration}ms to render!`);

    drawContext.putImageData(image, 0, 0);
};

export default app;
