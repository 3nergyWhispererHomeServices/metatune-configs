import type { AccountData } from "@cardinal/common";
import { fetchIdlAccount } from "@cardinal/common";
import type { Connection } from "@solana/web3.js";
import { PublicKey } from "@solana/web3.js";

import type { ConfigEntryData, CONFIGS_PROGRAM } from "./constants";
import { CONFIGS_IDL } from "./constants";
import { findConfigEntryId } from "./pda";

export const getConfigEntry = async (
  connection: Connection,
  key: string,
  configAccount = PublicKey.default
): Promise<AccountData<ConfigEntryData>> => {
  const configEntryId = findConfigEntryId(key, configAccount);
  return fetchIdlAccount<"configEntry", CONFIGS_PROGRAM>(
    connection,
    configEntryId,
    "configEntry",
    CONFIGS_IDL
  );
};
