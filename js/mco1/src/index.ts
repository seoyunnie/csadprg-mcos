// oxlint-disable no-await-in-loop

import { CURRENCY_CODES, exchangeCurrencies, setExchangeRates } from "./currency.ts";
import { printOrderedList, prompt } from "./io-utils.ts";
import { Account } from "./banking.ts";

const TRANSACTION_TITLES = Object.freeze([
  "Register Account Name",
  "Deposit Amount",
  "Withdraw Amount",
  "Currency Exchange",
  "Record Exchange Rates",
  "Show Interest Amount",
] as const);

const accounts: Account[] = [];
const currencyToRate = new Map<string, number>();

for (const code of CURRENCY_CODES.values().drop(1)) {
  currencyToRate.set(code, 1);
}

mainMenu: while (true) {
  console.log("Select Transaction:");
  printOrderedList(TRANSACTION_TITLES);

  console.log();

  let chosenIdx: number;

  try {
    chosenIdx = Number.parseInt(await prompt("> "), 10);
  } catch {
    chosenIdx = 0;
  }

  console.log();

  if (chosenIdx > 0 && chosenIdx <= TRANSACTION_TITLES.length) {
    console.log(TRANSACTION_TITLES[chosenIdx - 1]);
  }

  switch (chosenIdx) {
    case 1: {
      const account = new Account(await prompt("Account Name: "));

      if (accounts.some((a) => a.name === account.name)) {
        console.log("An account with this name already exists!");
      } else {
        accounts.push(account);
      }

      break;
    }
    case 2:
    // oxlint-disable-next-line no-magic-numbers
    case 3: {
      const accountName = await prompt("Account Name: ");
      const account = accounts.find((a) => a.name === accountName);

      if (account) {
        if (chosenIdx === 2) {
          await account.depositBalance(currencyToRate);
        } else {
          await account.withdrawBalance(currencyToRate);
        }
      } else {
        console.log("No account with this name exists!");
      }

      break;
    }
    // oxlint-disable-next-line no-magic-numbers
    case 4:
      currencyExchange: while (true) {
        await exchangeCurrencies(currencyToRate);

        console.log();

        while (true) {
          let isRepeating = await prompt("Convert another currency? (Y/N): ");
          isRepeating = isRepeating.toUpperCase();

          if (isRepeating === "Y") {
            console.log();

            break;
          } else if (isRepeating === "N") {
            break currencyExchange;
          }

          console.log("Only accepting a [Y]es or [N]o answer!");

          console.log();
        }
      }

      break;
    // oxlint-disable-next-line no-magic-numbers
    case 5:
      console.log();

      await setExchangeRates(currencyToRate);

      break;
    // oxlint-disable-next-line no-magic-numbers
    case 6: {
      const accountName = await prompt("Account Name: ");
      const account = accounts.find((a) => a.name === accountName);

      if (account) {
        await account.calculateInterest();
      } else {
        console.log("No account with this name exists!");
      }

      break;
    }
    default:
      console.log("No transaction with this ID exists!");

      break;
  }

  console.log();

  while (true) {
    let isContinuing = await prompt("Back to the Main Menu (Y/N): ");
    isContinuing = isContinuing.toUpperCase();

    if (isContinuing === "Y") {
      console.log();

      break;
    } else if (isContinuing === "N") {
      break mainMenu;
    }

    console.log("Only accepting a [Y]es or [N]o answer!");

    console.log();
  }
}
