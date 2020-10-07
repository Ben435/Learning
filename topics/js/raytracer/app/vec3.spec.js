import Vec3 from './vec3';

describe('vec3', () => {
    it('normalize', () => {
        const tvec = new Vec3(1, 2, 3);

        tvec.normalize();

        expect(tvec.length2()).toEqual(1);
    })
})
