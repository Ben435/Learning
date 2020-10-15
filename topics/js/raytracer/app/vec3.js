import { clamp } from './math';

const valToColor = (val) => {
    return clamp(val, 1, 0) * 255;
}

export class Vec3 {
    constructor(x=0, y=0, z=0) {
        this.x = x;
        this.y = y;
        this.z = z;
    }

    add(other) {
        if (other instanceof Vec3) {
            return new Vec3(this.x + other.x, this.y + other.y, this.z + other.z);
        } else {
            return new Vec3(this.x + other, this.y + other, this.z + other);
        }
    }

    sub(other) {
        if (other instanceof Vec3) {
            return new Vec3(this.x - other.x, this.y - other.y, this.z - other.z);
        } else {
            return new Vec3(this.x - other, this.y - other, this.z - other);
        }
    }

    mul(other) {
        if (other instanceof Vec3) {
            return new Vec3(this.x * other.x, this.y * other.y, this.z * other.z);
        } else {
            return new Vec3(this.x * other, this.y * other, this.z * other);
        }
    }

    dot(other) {
        return this.x * other.x + this.y * other.y + this.z * other.z;
    }

    length2() {
        return this.x * this.x + this.y * this.y + this.z * this.z;
    }

    length() {
        return Math.sqrt(this.length2())
    }

    normalize() {
        const normal2 = this.length2();
        if (normal2 > 0) {
            const invNormal = 1 / Math.sqrt(normal2);
            this.x *= invNormal;
            this.y *= invNormal;
            this.z *= invNormal;
        }
        return this;
    }

    reflect(normal) {
        return this.sub(normal.mul(2).mul(this.dot(normal)));
    }

    invert() {
        return this.mul(-1);
    }

    isZero() {
        return this.x === 0 && this.y === 0 && this.z === 0;
    }

    toColor(opacity=1) {
        return  [valToColor(this.x), valToColor(this.y), valToColor(this.z), opacity * 255]
    }

    isNaN() {
        return !(Number.isFinite(this.x) && Number.isFinite(this.y) && Number.isFinite(this.z));
    }

    toString() {
        return `Vec(x=${this.x},y=${this.y},z=${this.z})`;
    }
}

export default Vec3;
