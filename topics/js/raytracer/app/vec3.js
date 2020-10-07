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
        const len2 = this.length2;
        if (len2 === 1) {
            return 1;
        }
        return Math.sqrt();
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
}

export default Vec3;
