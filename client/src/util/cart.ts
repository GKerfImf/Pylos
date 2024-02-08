export default function cartesian<T>(a: T[], b: T[]): T[][] {
  return ([] as T[][]).concat(
    ...a.map((a2) => b.map((b2) => ([] as T[]).concat(a2, b2)))
  );
}
