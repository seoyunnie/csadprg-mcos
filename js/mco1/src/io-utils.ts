import process from "node:process";
import readline from "node:readline/promises";

export function printOrderedList<T extends { toString(): string }>(arr: readonly T[]): void {
  for (const [i, elm] of arr.entries()) {
    console.log(`[${i + 1}] ${elm.toString()}`);
  }
}

export async function prompt(msg: string): Promise<string> {
  const rl = readline.createInterface({ input: process.stdin, output: process.stdout });

  const userIn = await rl.question(msg);

  rl.close();

  return userIn;
}
