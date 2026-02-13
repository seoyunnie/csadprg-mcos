export function stringToNumber(str: string): number {
  const num = Number(str);

  if (Number.isNaN(num) || !Number.isFinite(num)) {
    throw new TypeError(`${str} is not a finite number`);
  }

  return num;
}
