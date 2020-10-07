import Vec3 from './vec3';
import Sphere from './sphere';

const width = 640;
const height = 480;

const spheres = [
    new Sphere(new Vec3(width/2, height/2), 100, new Vec3(255, 255, 0), new Vec3(0, 0, 0), 1, 0),
    new Sphere(new Vec3(width/2 - 200, height/2 + 60), 70, new Vec3(255, 0, 255), new Vec3(0, 0, 0), 1, 0),
    new Sphere(new Vec3(50, 50), 20, new Vec3(0, 0, 0), new Vec3(255, 255, 255), 1, 0)
]
const maxDepth = 1;

const app = () => {
    const canvas = document.getElementById("drawspace");
    const renderButton = document.getElementById("renderButton");

    canvas.setAttribute("width", width);
    canvas.setAttribute("height", height);

    const context = canvas.getContext("2d");

    renderButton.onclick = () => render(context);
}

const render = (drawContext) => {
    const image = drawContext.createImageData(width, height);

    const invWidth = 1 / width;
    const invHeight = 1 / height;
    const fov = 30;
    const aspectRatio = width / height;
    const angle = Math.tan(Math.PI * 0.5 * fov / 180);

    const startTime = performance.now();
    for (let y=0; y<height; y++) {
        for (let x=0; x<width; x++) {
            const rayOrigin = new Vec3(x, y, 0);
            const xx = (2 * ((x + 0.5) * invWidth) - 1) * angle * aspectRatio;
            const yy = (1 - 2 * ((y + 0.5) * invHeight)) * angle;
            const rayDirection = new Vec3(xx, yy, -1).normalize();

            const pixel = trace(rayOrigin, rayDirection, spheres, 0);

            const index = (y * width + x) * 4;
            
            try {
                image.data.set(pixel, index);
            } catch {
                console.error('Catch', x, y, index);
                break;
            }
        }
    }
    const endTime = performance.now();

    const duration = endTime - startTime;

    console.log(`Took ${duration}ms to render!`);

    drawContext.putImageData(image, 0, 0);
};

const trace = (rayOrigin, rayDirection, geometries, currentDepth) => {
    const [geo, _distance] = geometries.map(geo => {
        const [intersected, nearPoint] = geo.intersect(rayOrigin, rayDirection);

        return [intersected, nearPoint, geo];
    }).reduce(([closest_geo, nearestPoint], [intersected, nearPoint, geo]) => {
        if (intersected) {
            if (nearPoint < nearestPoint) {
                return [geo, nearPoint]
            }
        }
        return [closest_geo, nearestPoint];
    }, [null, Number.POSITIVE_INFINITY]);

    if (geo) {
        return geo.getSurfaceColor();
    } else {
        return [255, 255, 255, 255];
    }
};

app();
