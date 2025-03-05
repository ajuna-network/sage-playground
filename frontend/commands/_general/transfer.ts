import { formatBalance } from "@/lib/format-balance";
import { Command } from "@/types/command";
import { MultiAddress } from "@polkadot-api/descriptors";

export const transfer: Command = {
  execute: async (
    args: string[],
    { activeSigner, selectedAccount, api, client }
  ) => {
    if (!api) return "No chain connection available";
    if (!activeSigner || !selectedAccount)
      return "Please connect and select an account first";

    const chainInfo = await client?.getChainSpecData();

    if (args.length !== 2) return "Invalid number of arguments (to, amount)";

    const [dest, rawAmount] = args;

    // Convert float string to chain units, handling decimals
    const decimals = chainInfo?.properties.tokenDecimals;
    const [whole, fraction = "0"] = rawAmount.split(".");
    const amount =
      BigInt(whole) * BigInt(10 ** decimals) +
      BigInt(fraction.padEnd(decimals, "0").slice(0, decimals));

    const tx = api.tx.Balances.transfer_keep_alive({
      dest: MultiAddress.Id(dest),
      value: BigInt(amount),
    });

    const result = await tx.signAndSubmit(activeSigner, { at: "best" });
    if (result.ok) {
      return `✅ Transferred ${amount} tokens (${formatBalance({
        value: amount,
        decimals: chainInfo?.properties.tokenDecimals,
        unit: chainInfo?.properties.tokenSymbol,
        options: {
          nDecimals: 4,
        },
      })}) to ${dest}`;
    } else {
      console.log("result", result);
      return `❌ Transfer failed`;
    }
  },
  help: {
    command: "transfer [to] [amount]",
    description: "Transfer tokens to an address",
  },
};
