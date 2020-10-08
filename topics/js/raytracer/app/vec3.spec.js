import Vec3 from './vec3';

describe('vec3', () => {
    it('normalize', () => {
        const tvec = new Vec3(1, 2, 3);

        tvec.normalize();

        expect(tvec.length2()).toEqual(1);
    })

    it('reflect', () => {
        const tvec = new Vec3(1, 1, 0);
        const normalVec = new Vec3(0, 1, 0);

        const result = tvec.reflect(normalVec);

        expect(result).toEqual(new Vec3(2, 0, 0));
    })
})
