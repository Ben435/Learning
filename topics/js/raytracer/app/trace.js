import Vec3 from './vec3';
import { mix } from './math';
import { maxDepth, MODE } from './constants';

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
        return new Vec3(1.0, 1.0, 1.0);
    } else if (options.mode === MODE.FLAT) {
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

    let inside = false;
    if (rayDirection.dot(normal) > 0) {
        normal = normal.invert();
        inside = true;
    }
    // Bias, black magic
    const bias = 0.0001;

    // if in diffuse mode, or no more bounces allowed, or geometry is diffuse (no reflections or transmission)
    if (options.mode === MODE.DIFFUSE || currentDepth >= maxDepth || !(geo.reflectance > 0 || geo.transmission > 0)) {
        // Compute illumination
        const resultantColor = geometries
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
                // TODO: These should be tunable per light
                const attenuationVals = {
                    constant: 1.0,
                    linear: 0.09,
                    quadratic: 0.032,
                }
                const attenuation = 1 / (
                    attenuationVals.constant +
                    attenuationVals.linear * lightDistance +
                    attenuationVals.quadratic * lightDistance * lightDistance
                );

                return resultantColor.add(
                    geo.surfaceColor
                        .mul(transmission)
                        .mul(Math.max(0, normal.dot(shadowRayDirection)))
                        .mul(light.emissionColor.mul(attenuation)),
                    );
            }, new Vec3());

        return resultantColor.add(geo.emissionColor);
    }

    let reflection = new Vec3();
    let refraction = new Vec3();
    const facingRatio = rayDirection.invert().dot(normal);
    const fresnelEffect = mix(Math.pow(1 - facingRatio, 3), 1, 0.1);
    if (geo.reflectance > 0) {
        const reflectionRayDirection = rayDirection.reflect(normal).normalize();
        const reflectionRayOrigin = point.add(normal.mul(bias));

        try {
            reflection = trace(reflectionRayOrigin, reflectionRayDirection, geometries, currentDepth+1);
        } catch (e) {
            throw Error(`Error on reflection: ${reflectionRayOrigin}, ${reflectionRayDirection} -> ${e}`);
        }
    }
    if (geo.transmission > 0) {
        const ior = 1.1; // Index Of Refraction
        const eta = inside ? ior : 1 / ior;
        const cosi = normal.invert().dot(rayDirection);
        const k = 1 - eta * eta * (1 - cosi * cosi);
        if (k > 0) {
            const refractionRayDirection = rayDirection
                .mul(eta)
                .add(normal.mul(eta * cosi - Math.sqrt(k)))
                .normalize();
            const refractionRayOrigin = point.sub(normal.mul(bias));
            try {
                refraction = trace(refractionRayOrigin, refractionRayDirection, geometries, currentDepth+1);
            } catch (e) {
                throw Error(`Error on refraction: ${refractionRayOrigin}, ${refractionRayDirection} -> ${e}`);
            }
        } else {
            // Total internal reflection. 100% reflected, so dw about it.
            console.warn('Total internal reflection occurred, ignoring');
        }
    }

    return reflection
            .mul(fresnelEffect)
            .add(refraction
                .mul(1 - fresnelEffect)
                .mul(geo.transmission))
            .mul(geo.surfaceColor)
            .add(geo.emissionColor);
};

export default trace;
