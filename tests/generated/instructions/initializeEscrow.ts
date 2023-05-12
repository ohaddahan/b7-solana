/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as splToken from '@solana/spl-token'
import * as beet from '@metaplex-foundation/beet'
import * as web3 from '@solana/web3.js'
import {
  InitializeEscrowArgs,
  initializeEscrowArgsBeet,
} from '../types/InitializeEscrowArgs'

/**
 * @category Instructions
 * @category InitializeEscrow
 * @category generated
 */
export type InitializeEscrowInstructionArgs = {
  args: InitializeEscrowArgs
}
/**
 * @category Instructions
 * @category InitializeEscrow
 * @category generated
 */
export const initializeEscrowStruct = new beet.FixableBeetArgsStruct<
  InitializeEscrowInstructionArgs & {
    instructionDiscriminator: number[] /* size: 8 */
  }
>(
  [
    ['instructionDiscriminator', beet.uniformFixedSizeArray(beet.u8, 8)],
    ['args', initializeEscrowArgsBeet],
  ],
  'InitializeEscrowInstructionArgs'
)
/**
 * Accounts required by the _initializeEscrow_ instruction
 *
 * @property [_writable_, **signer**] signer
 * @property [_writable_] escrow
 * @property [] associatedTokenProgram
 * @category Instructions
 * @category InitializeEscrow
 * @category generated
 */
export type InitializeEscrowInstructionAccounts = {
  signer: web3.PublicKey
  escrow: web3.PublicKey
  systemProgram?: web3.PublicKey
  tokenProgram?: web3.PublicKey
  associatedTokenProgram: web3.PublicKey
  rent?: web3.PublicKey
  anchorRemainingAccounts?: web3.AccountMeta[]
}

export const initializeEscrowInstructionDiscriminator = [
  243, 160, 77, 153, 11, 92, 48, 209,
]

/**
 * Creates a _InitializeEscrow_ instruction.
 *
 * @param accounts that will be accessed while the instruction is processed
 * @param args to provide as instruction data to the program
 *
 * @category Instructions
 * @category InitializeEscrow
 * @category generated
 */
export function createInitializeEscrowInstruction(
  accounts: InitializeEscrowInstructionAccounts,
  args: InitializeEscrowInstructionArgs,
  programId = new web3.PublicKey('Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS')
) {
  const [data] = initializeEscrowStruct.serialize({
    instructionDiscriminator: initializeEscrowInstructionDiscriminator,
    ...args,
  })
  const keys: web3.AccountMeta[] = [
    {
      pubkey: accounts.signer,
      isWritable: true,
      isSigner: true,
    },
    {
      pubkey: accounts.escrow,
      isWritable: true,
      isSigner: false,
    },
    {
      pubkey: accounts.systemProgram ?? web3.SystemProgram.programId,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.tokenProgram ?? splToken.TOKEN_PROGRAM_ID,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.associatedTokenProgram,
      isWritable: false,
      isSigner: false,
    },
    {
      pubkey: accounts.rent ?? web3.SYSVAR_RENT_PUBKEY,
      isWritable: false,
      isSigner: false,
    },
  ]

  if (accounts.anchorRemainingAccounts != null) {
    for (const acc of accounts.anchorRemainingAccounts) {
      keys.push(acc)
    }
  }

  const ix = new web3.TransactionInstruction({
    programId,
    keys,
    data,
  })
  return ix
}