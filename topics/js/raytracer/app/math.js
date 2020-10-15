
export const clamp = (val, upper, lower) => {
    return Math.min(Math.max(val, lower), upper)
}

export const mix = (a, b, mixRatio) => b * mixRatio + a * (1 - mixRatio);
