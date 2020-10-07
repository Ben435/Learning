
const solveQuadratic = (a, b, c) => {
    const discriminant = b * b - 4 * a * c;

    if (discriminant < 0) {
        return [false, 0, 0];
    } else if (discriminant == 0) {
        let singlePoint = -0.5 * b / a;
        return [true, singlePoint, singlePoint];
    } else {
        const q = b > 0 ?
            -0.5 * (b + Math.sqrt(discriminant)) :
            -0.5 * (b - Math.sqrt(discriminant));
        const firstPoint =  q / a;
        const secondPoint = c / q;

        if (firstPoint > secondPoint) {
            return [true, firstPoint, secondPoint];
        } else {
            return [true, secondPoint, firstPoint];
        }
    }
}

export class Sphere {
    constructor(center, radius, surfaceColor, emissionColor, transparency, reflection) {
        this.center = center;
        this.radius = radius;
        this.surfaceColor = surfaceColor;
        this.emissionColor = emissionColor;
        this.transparency = transparency;
        this.reflection = reflection;
    }

    intersect(rayOrigin, rayDirection) {        
        const diff = rayOrigin.sub(this.center);

        const a = rayDirection.dot(rayDirection);
        const b = 2 * rayDirection.dot(diff);
        const c = diff.dot(diff) - (this.radius * this.radius)

        const [found, t0, t1] = solveQuadratic(a, b, c);
        if (!found) {
            return [false, 0];
        }
        let returnPoint = t0;
        if (returnPoint < 0) {
            returnPoint = t1;
            if (returnPoint < 0) {
                // Both points are negative, can't use either :(.
                return [false, 0];
            }
        }
        return [true, returnPoint];
    }

    getSurfaceColor() {
        return [this.surfaceColor.x, this.surfaceColor.y, this.surfaceColor.z, this.transparency * 255];
    }
}

export default Sphere;
