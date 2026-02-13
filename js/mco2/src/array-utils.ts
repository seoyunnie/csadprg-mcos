export function sum(arr: readonly number[]): number {
  let sum = 0;

  for (const num of arr) {
    sum += num;
  }

  return sum;
}
