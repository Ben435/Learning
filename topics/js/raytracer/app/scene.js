import Sphere from './sphere';
import Vec3 from './vec3';

const geometries = [
    // Platform
    new Sphere(new Vec3(0, -10004, -20), 10000, new Vec3(0.2, 0.2, 0.2), 0, 0, new Vec3()),

    // Scene
    new Sphere(new Vec3(0, 0, -20), 4, new Vec3(1, 0.32, 0.36), 1, 0.5, new Vec3()),
    new Sphere(new Vec3(5, -1, -15), 2, new Vec3(0.90, 0.76, 0.46), 1, 0, new Vec3()),
    new Sphere(new Vec3(5, 0, -25), 3, new Vec3(0.65, 0.77, 0.97), 1, 0, new Vec3()),
    new Sphere(new Vec3(-5.5, 0, -15), 3, new Vec3(0.90, 0.90, 0.90), 1, 0, new Vec3()),

    // Light
    new Sphere(new Vec3(0, 20, -10), 3, new Vec3(), 0, 0, new Vec3(3.0, 3.0, 3.0)),
];

export default geometries;
