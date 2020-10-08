import { solveQuadratic, default as Sphere } from './sphere';

describe('sphere', () => {
    it('solveQuadratic', () => {
        expect(solveQuadratic(1, 2, 3)).toEqual(null);
        expect(solveQuadratic(1, 2, 0)).toEqual({ t0: 0, t1: -2 });
        expect(solveQuadratic(2, 2, 0)).toEqual({ t0: 0, t1: -1 });
    })
});
