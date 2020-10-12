
export const clamp = (val, upper, lower) => {
    return Math.min(Math.max(val, lower), upper)
}
