import { printOrderedList, prompt } from "./io-utils.ts";

export const CURRENCY_TITLES = Object.freeze([
  "Philippine Peso (PHP)",
  "United States Dollar (USD)",
  "Japanese Yen (JPY)",
  "British Pound Sterling (GBP)",
  "Euro (EUR)",
  "Chinese Yuan Renminni (CNY)",
] as const);
// oxlint-disable-next-line no-magic-numbers
export const CURRENCY_CODES = Object.freeze(CURRENCY_TITLES.map((c) => c.slice(-4, -1)));

export const CURRENCY_ROUNDING_PRECISION = 2;

export function convertCurrency(
  amount: number,
  src: string,
  dest: string,
  currencyToRate: ReadonlyMap<string, number>,
): number {
  const srcPHPAmount = src === "PHP" ? amount : amount / currencyToRate.get(src)!;

  return dest === "PHP" ? srcPHPAmount : srcPHPAmount * currencyToRate.get(dest)!;
}

export async function exchangeCurrencies(currencyToRate: ReadonlyMap<string, number>): Promise<void> {
  console.log("Source Currency Options:");
  printOrderedList(CURRENCY_TITLES);

  console.log();

  let srcIdx: number;

  try {
    srcIdx = Number.parseInt(await prompt("Source Currency: "), 10) - 1;

    if (srcIdx < 0) {
      throw new TypeError(`${srcIdx} is not a positive integer`);
    }
  } catch {
    console.log("ID must be a positive whole number (integer)!");

    return;
  }

  if (srcIdx >= CURRENCY_TITLES.length) {
    console.log("No currency with this ID exists!");

    return;
  }

  let srcAmount: number;

  try {
    srcAmount = Number.parseFloat(await prompt("Source Amount: "));
  } catch {
    console.log("Amount must be a floating point number!");

    return;
  }

  console.log();

  console.log("Exchanged Currency Options:");
  printOrderedList(CURRENCY_TITLES);

  console.log();

  let exchangeIdx: number;

  try {
    exchangeIdx = Number.parseInt(await prompt("Exchange Currency: "), 10) - 1;

    if (exchangeIdx < 0) {
      throw new TypeError(`${exchangeIdx} is not a positive integer`);
    }
  } catch {
    console.log("ID must be a positive whole number (integer)!");

    return;
  }

  if (exchangeIdx >= CURRENCY_TITLES.length) {
    console.log("No currency with this ID exists!");

    return;
  }

  console.log(
    `Exchange Amount: ${convertCurrency(
      srcAmount,
      CURRENCY_CODES[srcIdx],
      CURRENCY_CODES[exchangeIdx],
      currencyToRate,
    ).toFixed(CURRENCY_ROUNDING_PRECISION)}`,
  );
}

export async function setExchangeRates(currencyToRate: Map<string, number>): Promise<void> {
  printOrderedList(CURRENCY_TITLES.slice(1));

  console.log();

  let idx: number;

  try {
    idx = Number.parseInt(await prompt("Select Foreign Currency: "), 10);

    if (idx < 0) {
      throw new TypeError(`${idx} is not a positive integer`);
    }
  } catch {
    console.log("ID must be a positive whole number (integer)!");

    return;
  }

  if (idx >= CURRENCY_TITLES.length) {
    console.log("No currency with this ID exists!");

    return;
  }

  let rate: number;

  try {
    rate = Number.parseFloat(await prompt("Exchange Rate: "));
  } catch {
    console.log("Amount must be a floating point number!");

    return;
  }

  currencyToRate.set(CURRENCY_CODES[idx], rate);
}
