const fixNegZero = num => {
    if (num === -num) {
        return 0;
    }
    return num;
}

export const solveQuadratic = (a, b, c) => {
    const discriminant = b * b - 4 * a * c;

    if (discriminant < 0) {
        return null;
    } else if (discriminant == 0) {
        let singlePoint = fixNegZero(-0.5 * b / a);
        return {
            t0: singlePoint,
            t1: singlePoint,
        };
    } else {
        const q = b > 0 ?
            -0.5 * (b + Math.sqrt(discriminant)) :
            -0.5 * (b - Math.sqrt(discriminant));
        const firstPoint =  fixNegZero(q / a);
        const secondPoint = fixNegZero(c / q);

        if (firstPoint > secondPoint) {
            return {
                t0: firstPoint,
                t1: secondPoint,
            }
        } else {
            return {
                t0: secondPoint,
                t1: firstPoint,
            }
        }
    }
}

export class Sphere {
    constructor(center, radius, surfaceColor, transmission, reflectance, emissionColor) {
        this.center = center;
        this.radius = radius;

        this.surfaceColor = surfaceColor;

        this.transmission = transmission;
        this.reflectance = reflectance;

        this.emissionColor = emissionColor;
        this.emissionBrightness = 1.0;
    }

    intersect(rayOrigin, rayDirection) {        
        const length = this.center.sub(rayOrigin);
        const radius2 = this.radius * this.radius;

        const tca = length.dot(rayDirection);
        if (tca < 0) {
            return null;
        }
        const d2 = length.dot(length) - tca * tca;
        if (d2 > radius2) {
            return null;
        }
        const thc = Math.sqrt(radius2 - d2);

        return {
            t0: tca - thc,
            t1: tca + thc,
        }
    }
}

export default Sphere;
