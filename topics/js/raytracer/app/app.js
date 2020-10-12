import Vec3 from './vec3';
import Scene from './scene';

const width = 640;
const height = 480;
const maxDepth = 5;

const app = () => {
    const canvas = document.getElementById("drawspace");
    const renderButton = document.getElementById("renderButton");

    canvas.setAttribute("width", width);
    canvas.setAttribute("height", height);

    const context = canvas.getContext("2d");
    const options = {
        flat: false,
    }

    renderButton.onclick = async() => {
        renderButton.setAttribute("disabled", true);

        await render(context, options);

        renderButton.removeAttribute("disabled");
    }
}

const render = async (drawContext, options={}) => {
    const image = drawContext.createImageData(width, height);
    const geometries = Scene;

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

            let pixel;
            try {
                pixel = trace(rayOrigin, rayDirection, geometries, 0, options);
                if (pixel.isNaN()) {
                    throw Error(`Pixel returned NaN: ${pixel}`);
                }
            } catch (e) {
                throw Error(`Error tracing ray x=${x} y=${y} rayOrigin=${rayOrigin} rayDirection=${rayDirection} -> ${e} `);
            }

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

const mix = (a, b, mixRatio) => b * mixRatio + a * (1 - mixRatio)

const trace = (rayOrigin, rayDirection, geometries, currentDepth, options={}) => {
    if (rayOrigin.isNaN() || rayDirection.isNaN()) {
        throw Error(`Origin || Direction is NaN: ro=${rayOrigin} rd=${rayDirection}`);
    }

    let tnearest = Number.POSITIVE_INFINITY;
    let geo = null;

    geometries.forEach(geometry => {
        const result = geometry.intersect(rayOrigin, rayDirection);

        if (result) {
            let { t0, t1 } = result;
            if (t0 < 0) {
                t0 = t1;
            }

            if (t0 < tnearest) {
                tnearest = t0;
                geo = geometry;
            }
        }
    });

    if (!geo) {
        // TODO: Background color goes here. Currents #fff -> white
        return new Vec3(255, 255, 255);
    } else if (options.flat) {
        // No light sim, just render surface color (for positional debugging)
        return geo.surfaceColor;
    }

    const point = rayOrigin.add(rayDirection.mul(tnearest));
    let normal = point.sub(geo.center).normalize();

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
    // Bias, black magic
    const bias = 0.0001;

    // if ((geo.reflectance > 0 || geo.opacity < 1) && currentDepth < maxDepth) {
    //     let reflection = new Vec3();
    //     let refraction = new Vec3();
    //     const facingRatio = rayDirection.invert().dot(normal);
    //     const fresnelEffect = mix(Math.pow(1 - facingRatio, 3), 1, 0.1);
    //     if (geo.reflectance > 0) {
    //         const reflectionRayDirection = rayDirection.reflect(normal).normalize();
    //         const reflectionRayOrigin = point.add(normal.mul(bias));
    
    //         try {
    //             reflection = trace(reflectionRayOrigin, reflectionRayDirection, geometries, currentDepth+1);
    //         } catch (e) {
    //             throw Error(`Error on reflection: ${reflectionRayOrigin}, ${reflectionRayDirection} -> ${e}`);
    //         }
    //     }
    //     if (false && geo.opacity < 1) {
    //         const ior = 1.1; // Index Of Refraction
    //         const eta = inside ? ior : 1 / ior;
    //         const cosi = normal.invert().dot(rayDirection);
    //         // Dark arts start here
    //         // TODO: For some reason, k is negative. I suspect cosi needs to be smaller.
    //         const k = 1 - eta * eta * (1 - cosi * cosi);
    //         if (k < 0) {
    //             throw Error(`Negative k k=${k} eta=${eta} cosi=${cosi} normal=${normal} inormal=${normal.invert()} raydir=${rayDirection} nraydir=${rayDirection.normalize()}`)
    //         }
    //         const refractionRayDirection = rayDirection.mul(eta).add(normal.mul(eta * cosi - Math.sqrt(k))).normalize();
    //         const refractionRayOrigin = point.sub(normal);
    //         // Dark arts end here
    //         try {
    //             refraction = trace(refractionRayOrigin, refractionRayDirection, geometries, currentDepth+1);
    //         } catch (e) {
    //             throw Error(`Error on refraction: ${refractionRayOrigin}, ${refractionRayDirection} -> ${e}`);
    //         }
    //     }

    //     resultantColor = reflection
    //             .mul(fresnelEffect)
    //             .add(refraction.mul(1 - fresnelEffect).mul(1 - geo.opacity))
    //             .mul(geo.surfaceColor);
    // } else {
    // Compute illumination
    resultantColor = geometries
        .filter(geo => !geo.emissionColor.isZero())
        .reduce((resultantColor, light) => {
            const shadowRayOrigin = point.add(normal.mul(bias));
            const toLight = light.center.sub(point);
            const shadowRayDirection = toLight.normalize();
            let transmission = 1;

            const inShadow = geometries
                .filter(otherGeo => otherGeo !== light)
                .find(otherGeo => otherGeo.intersect(shadowRayOrigin, shadowRayDirection));

            if (inShadow) {
                transmission = 0;
            }

            const lightDistance = Math.abs(toLight.length())
            const attenuationVals = {
                constant: 1.0,
                linear: 0.35,
                quadratic: 0.44,
            }
            const attenuation = 1 / (
                attenuationVals.constant +
                attenuationVals.linear * lightDistance +
                attenuationVals.quadratic * lightDistance * lightDistance
            );
            // const attenuation = 1;

            return resultantColor.add(
                geo.surfaceColor
                    .mul(transmission)
                    .mul(Math.max(0, normal.dot(shadowRayDirection)))
                    .mul(light.emissionColor.mul(attenuation)),
                );
        }, new Vec3());
    
    return resultantColor.add(geo.emissionColor);
};

export default app;
