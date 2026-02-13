import { convertCurrency, CURRENCY_CODES, CURRENCY_ROUNDING_PRECISION } from "./currency.ts";
import { prompt } from "./io-utils.ts";

export class Account {
  static readonly ANNUAL_INTEREST_RATE = 0.05;

  readonly name: string;
  balance = 0;
  readonly currency = "PHP";

  constructor(name: string) {
    this.name = name;
  }

  async depositBalance(currencyToRate: ReadonlyMap<string, number>): Promise<void> {
    console.log(`Current Balance: ${this.balance.toFixed(CURRENCY_ROUNDING_PRECISION)}`);

    let currency = await prompt("Currency: ");
    currency = currency.toUpperCase();

    if (!CURRENCY_CODES.some((c) => c === currency)) {
      console.log("No currency with this code exists!");

      return;
    }

    console.log();

    let amount: number;

    try {
      amount = Number.parseFloat(await prompt("Deposit Amount: "));
    } catch {
      console.log("Deposit amount must be a floating point number!");

      return;
    }

    this.balance += currency === "PHP" ? amount : convertCurrency(amount, currency, "PHP", currencyToRate);

    console.log(`Updated Balance: ${this.balance.toFixed(CURRENCY_ROUNDING_PRECISION)}`);
  }

  async withdrawBalance(currencyToRate: ReadonlyMap<string, number>): Promise<void> {
    console.log(`Current Balance: ${this.balance.toFixed(CURRENCY_ROUNDING_PRECISION)}`);

    let currency = await prompt("Currency: ");
    currency = currency.toUpperCase();

    if (!CURRENCY_CODES.some((c) => c === currency)) {
      console.log("No currency with this code exists!");

      return;
    }

    console.log();

    let amount;

    try {
      amount = Number.parseFloat(await prompt("Withdraw Amount: "));
    } catch {
      console.log("Withdraw amount must be a floating point number!");

      return;
    }

    const updatedBalance =
      this.balance - (currency === "PHP" ? amount : convertCurrency(amount, currency, "PHP", currencyToRate));

    if (updatedBalance < 0) {
      console.log("Withdraw amount must be less than the current balance!");

      return;
    }

    this.balance = updatedBalance;

    console.log(`Updated Balance: ${this.balance.toFixed(CURRENCY_ROUNDING_PRECISION)}`);
  }

  async calculateInterest(): Promise<void> {
    let { balance } = this;

    console.log(`Current Balance: ${balance.toFixed(CURRENCY_ROUNDING_PRECISION)}`);
    console.log(`Currency: ${this.currency}`);
    console.log(`Interest Rate: ${(Account.ANNUAL_INTEREST_RATE * 100).toFixed(CURRENCY_ROUNDING_PRECISION)}%`);

    console.log();

    let dayCnt: number;

    try {
      dayCnt = Number.parseInt(await prompt("Total Number of Days: "), 10);

      if (dayCnt < 0) {
        throw new TypeError(`${dayCnt} is not a positive integer`);
      }
    } catch {
      console.log("Number must be a positive whole number (integer)!");

      return;
    }

    console.log();

    console.log("Day | Interest | Balance |");

    const dailyInterest = Math.round(balance * (Account.ANNUAL_INTEREST_RATE / 365) * 100) / 100;

    for (let i = 1; i <= dayCnt; i++) {
      balance += dailyInterest;

      console.log(
        // oxlint-disable-next-line no-magic-numbers
        `${String(i).padEnd(3)} | ${String(dailyInterest).padEnd(8)} | ${String(balance.toFixed(CURRENCY_ROUNDING_PRECISION)).padEnd(7)} |`,
      );
    }
  }
}
