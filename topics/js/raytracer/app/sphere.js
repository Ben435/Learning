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
    constructor(center, radius, surfaceColor, emissionColor, emissionBrightness, opacity, reflectance) {
        this.center = center;
        this.radius = radius;
        this.surfaceColor = surfaceColor;
        this.emissionColor = emissionColor;
        this.emissionBrightness = emissionBrightness;
        this.opacity = opacity;
        this.reflectance = reflectance;
    }

    intersect(rayOrigin, rayDirection) {        
        const diff = rayOrigin.sub(this.center);

        const a = rayDirection.dot(rayDirection);
        const b = 2 * rayDirection.dot(diff);
        const c = diff.dot(diff) - (this.radius * this.radius)

        const found = solveQuadratic(a, b, c);
        if (!found) {
            return null;
        }
        const { t0, t1 } = found;
        let closestPoint = t0;
        if (closestPoint < 0) {
            closestPoint = t1;
            if (closestPoint < 0) {
                // Both points are negative, can't use either :(.
                return null;
            }
        }

        const point = rayOrigin.add(rayDirection.mul(closestPoint));
        const normal = point.sub(this.center).normalize();

        return {
            point,
            normal,
        };
    }
}

export default Sphere;
