#![allow(dead_code)]
#![allow(unused_variables)]

use crate::errors::errors::Errors;
use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use anchor_lang::solana_program::program_memory::sol_memset;

pub fn transfer_sol_from_pda(
    from: &mut AccountInfo,
    to: &mut AccountInfo,
    amount: u64,
) -> Result<()> {
    let post_from = from
        .lamports()
        .checked_sub(amount)
        .ok_or(Errors::NumericalOverflow)?;
    let post_to = to
        .lamports()
        .checked_add(amount)
        .ok_or(Errors::NumericalOverflow)?;

    **from.try_borrow_mut_lamports().unwrap() = post_from;
    **to.try_borrow_mut_lamports().unwrap() = post_to;
    Ok(())
}

pub fn close_account(from: &mut AccountInfo, to: &mut AccountInfo) -> Result<()> {
    let amount = from.lamports();
    let size = from.try_data_len()?;
    transfer_sol_from_pda(from, to, amount)?;
    sol_memset(&mut from.try_borrow_mut_data()?, 0, size);
    Ok(())
}

pub fn is_native(token_mint: &AccountInfo) -> bool {
    token_mint.key() == spl_token::native_mint::id()
}

pub fn transfer_sol<'a>(
    from: AccountInfo<'a>,
    to: AccountInfo<'a>,
    system_program: AccountInfo<'a>,
    amount: u64,
) -> Result<()> {
    solana_program::program::invoke(
        &solana_program::system_instruction::transfer(&from.key(), &to.key(), amount),
        &[from, to, system_program],
    )?;
    Ok(())
}

pub fn transfer_token<'a>(
    from: AccountInfo<'a>,
    to: AccountInfo<'a>,
    token_program: AccountInfo<'a>,
    owner: AccountInfo<'a>,
    amount: u64,
) -> Result<()> {
    solana_program::program::invoke(
        &spl_token::instruction::transfer(
            &token_program.key(),
            &from.key(),
            &to.key(),
            &owner.key(),
            &[],
            amount,
        )?,
        &[from, to, token_program, owner],
    )?;
    Ok(())
}

pub fn transfer_token_pda<'a>(
    from: AccountInfo<'a>,
    to: AccountInfo<'a>,
    token_program: AccountInfo<'a>,
    owner: AccountInfo<'a>,
    amount: u64,
    seeds: &[&[&[u8]]],
) -> Result<()> {
    solana_program::program::invoke_signed(
        &spl_token::instruction::transfer(
            &token_program.key(),
            &from.key(),
            &to.key(),
            &owner.key(),
            &[],
            amount,
        )?,
        &[from, to, token_program, owner],
        seeds,
    )?;
    Ok(())
}
