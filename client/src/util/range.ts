const range = (start: number, end: number): number[] => Array.from({ length: end - start }, (v, k) => k + start);

export default range;
