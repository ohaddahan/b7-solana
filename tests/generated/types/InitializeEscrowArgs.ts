/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as web3 from '@solana/web3.js'
import * as beet from '@metaplex-foundation/beet'
import * as beetSolana from '@metaplex-foundation/beet-solana'
export type InitializeEscrowArgs = {
  uuid: string
  side1Mint: web3.PublicKey
  side1Amount: beet.bignum
  side2Mint: web3.PublicKey
  side2Amount: beet.bignum
}

/**
 * @category userTypes
 * @category generated
 */
export const initializeEscrowArgsBeet =
  new beet.FixableBeetArgsStruct<InitializeEscrowArgs>(
    [
      ['uuid', beet.utf8String],
      ['side1Mint', beetSolana.publicKey],
      ['side1Amount', beet.u64],
      ['side2Mint', beetSolana.publicKey],
      ['side2Amount', beet.u64],
    ],
    'InitializeEscrowArgs'
  )