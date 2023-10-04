import Vec3 from './vec3';
import { mix } from './math';
import { maxDepth, MODE } from './constants';

const trace = (rayOrigin, rayDirection, geometries, currentDepth, options={}) => {
    const { geo, tnearest } = geometries.reduce((prev, geometry) => {
        const result = geometry.intersect(rayOrigin, rayDirection);

        if (result) {
            let { t0, t1 } = result;
            if (t0 < 0) {
                t0 = t1;
            }

            if (t0 < prev.tnearest) {
                return {
                    tnearest: t0,
                    geo: geometry
                };
            }
        }
        return prev;
    }, { tnearest: Number.POSITIVE_INFINITY, geo: null });

    if (!geo) {
        // TODO: Background color goes here. Currents #fff -> white
        return new Vec3(1.0, 1.0, 1.0);
    } else if (options.mode === MODE.FLAT) {
        // No light sim, just render surface color (for positional debugging)
        return geo.surfaceColor;
    }

    const point = rayOrigin.add(rayDirection.mul(tnearest));
    let normal = point.sub(geo.center).normalize();

    let inside = false;
    if (rayDirection.dot(normal) > 0) {
        normal = normal.invert();
        inside = true;
    }
    // Bias, not strictly needed in Js as its double precision, but good practice.
    const bias = 1e-4;

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

        reflection = trace(reflectionRayOrigin, reflectionRayDirection, geometries, currentDepth+1);
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
            refraction = trace(refractionRayOrigin, refractionRayDirection, geometries, currentDepth+1);
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
