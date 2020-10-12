import Vec3 from './vec3';

describe('vec3', () => {
    it('normalize', () => {
        const tvec = new Vec3(1, 2, 3);

        tvec.normalize();

        expect(tvec.length2()).toEqual(1);
    });

    it('sub', () => {
        const tvec = new Vec3(2, 2, 1);
        const otherVec = new Vec3(6, 3, 2);

        expect(otherVec.sub(tvec)).toEqual(new Vec3(4, 1, 1));
    });
})
