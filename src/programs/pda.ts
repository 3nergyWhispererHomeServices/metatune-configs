import { utils } from "@project-serum/anchor";
import { PublicKey } from "@solana/web3.js";

import { CONFIG_ENTRY_SEED, CONFIGS_ADDRESS } from "./constants";

/**
 * Finds the config entry id.
 * @returns
 */
export const findConfigEntryId = (
  key: string,
  configAccount = PublicKey.default
): PublicKey => {
  return PublicKey.findProgramAddressSync(
    [
      utils.bytes.utf8.encode(CONFIG_ENTRY_SEED),
      utils.bytes.utf8.encode(key),
      (configAccount ?? PublicKey.default).toBuffer(),
    ],
    CONFIGS_ADDRESS
  )[0];
};