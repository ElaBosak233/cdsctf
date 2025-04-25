export function curve(s: number, r: number, d: number, x: number): number {
    const ratio = r / s;
    const result = Math.floor(
        s * (ratio + (1 - ratio) * Math.exp((1 - x) / d))
    );
    return Math.min(result, s);
}

export function getOrdinal(n: number) {
    const s = ["th", "st", "nd", "rd"];
    const v = n % 100;
    return n + (s[(v - 20) % 10] || s[v] || s[0]);
}
