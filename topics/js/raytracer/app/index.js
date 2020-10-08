import Vec3 from './vec3';
import Sphere from './sphere';

const width = 640;
const height = 480;

const spheres = [
    new Sphere(new Vec3(width/2, height/2), 100, new Vec3(255, 255, 0), new Vec3(0, 0, 0), 0, 0.6, 0.2),
    new Sphere(new Vec3(width/2 - 200, height/2 + 60), 70, new Vec3(255, 0, 255), new Vec3(0, 0, 0), 0, 0.6, 0.2),
    new Sphere(new Vec3(200, 150), 20, new Vec3(0, 0, 255), new Vec3(0, 0, 0), 0, 1, 0.6),
    new Sphere(new Vec3(50, 50), 20, new Vec3(0, 0, 0), new Vec3(200, 200, 200), 1, 0, 0),
];
const maxDepth = 5;

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

            let pixel;
            try {
                pixel = trace(rayOrigin, rayDirection, spheres, 0);
            } catch (e) {
                throw Error(`Error tracing ray x=${x} y=${y} rayOrigin=${rayOrigin} rayDirection=${rayDirection} -> ${e} `);
            }

            const index = (y * width + x) * 4;
            
            try {
                image.data.set(pixel.toColor(), index);
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

const mix = (a, b, mixRatio) => b * mixRatio + a * (1 - mixRatio)

const trace = (rayOrigin, rayDirection, geometries, currentDepth) => {
    if (rayOrigin.isNaN() || rayDirection.isNaN()) {
        throw Error(`Origin || Direction is NaN: ro=${rayOrigin} rd=${rayDirection}`);
    }
    const result = geometries.map(geo => {
        const intersectResult = geo.intersect(rayOrigin, rayDirection);

        if (intersectResult) {
            const {
                point, 
                normal,
                distance,
            } = intersectResult;
            return {
                geo, 
                point,
                normal,
                distance,
            }
        }
    }).reduce((closestResult, interectResult) => {
        if (interectResult) {
            const {
                geo,
                point,
                normal,
                distance,
            } = interectResult;

            if (!closestResult || distance < closestResult.distance) {
                return {
                    geo,
                    point,
                    normal,
                    distance,
                }
            }
        }
        return closestResult;
    }, null);

    if (!result) {
        // TODO: Background color goes here.
        return new Vec3(255, 255, 255);
    }

    const {
        geo,
        point,
    } = result;
    let normal = result.normal.normalize();

    if (point.isNaN()) {
        throw Error("Point is NaN", result);
    } else if (normal.isNaN()) {
        throw Error("Normal is NaN", result);
    }

    let resultantColor = new Vec3();
    let inside = false;
    if (rayDirection.dot(normal) > 0) {
        normal = normal.invert();
        inside = true;
    }

    if ((geo.reflectance > 0 || geo.opacity < 1) && currentDepth < maxDepth) {
        let reflection = new Vec3();
        let refraction = new Vec3();
        const facingRatio = rayDirection.invert().dot(normal);
        const fresnelEffect = mix(Math.pow(1 - facingRatio, 3), 1, 0.1);
        if (geo.reflectance > 0) {
            const reflectionRayDirection = rayDirection.reflect(normal).normalize();
            const reflectionRayOrigin = point;
    
            try {
                reflection = trace(reflectionRayOrigin, reflectionRayDirection, geometries, currentDepth+1);
            } catch (e) {
                throw Error(`Error on reflection: ${reflectionRayOrigin}, ${reflectionRayDirection} -> ${e}`);
            }
        }
        if (geo.opacity < 1) {
            const ior = 1.1; // Index Of Refraction
            const eta = inside ? ior : 1 / ior;
            const cosi = normal.invert().dot(rayDirection);
            // Dark arts start here
            // TODO: For some reason, k is negative. I suspect cosi needs to be smaller.
            const k = 1 - eta * eta * (1 - cosi * cosi);
            if (k < 0) {
                throw Error(`Negative k k=${k} eta=${eta} cosi=${cosi} normal=${normal} inormal=${normal.invert()} raydir=${rayDirection} nraydir=${rayDirection.normalize()}`)
            }
            const refractionRayDirection = rayDirection.mul(eta).add(normal.mul(eta * cosi - Math.sqrt(k))).normalize();
            const refractionRayOrigin = point.sub(normal);
            // Dark arts end here
            try {
                refraction = trace(refractionRayOrigin, refractionRayDirection, geometries, currentDepth+1);
            } catch (e) {
                throw Error(`Error on refraction: ${refractionRayOrigin}, ${refractionRayDirection} -> ${e}`);
            }
        }

        resultantColor = reflection
                .mul(fresnelEffect)
                .add(refraction.mul(1 - fresnelEffect).mul(1 - geo.opacity))
                .mul(geo.surfaceColor);
    } else {
        // Compute illumination
        resultantColor = geometries
            .filter(geo => !geo.emissionColor.isZero())
            .reduce((resultantColor, light) => {
                const shadowRayDirection = light.center.sub(point);

                const inShadow = geometries
                    .filter(otherGeo => otherGeo !== light)
                    .find(otherGeo => otherGeo.intersect(point, shadowRayDirection));

                if (inShadow) {
                    return resultantColor; // No effect
                } else {
                    // TODO: mix colors of light + surface + brightness

                    return geo.surfaceColor.add(light.emissionColor.mul(light.emissionBrightness));
                }
            }, new Vec3());
    }
    
    return resultantColor.add(geo.emissionColor);
};

app();
