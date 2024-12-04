use borsh::{BorshDeserialize, BorshSerialize};
use num_derive::FromPrimitive;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    program_pack::{IsInitialized, Pack, Sealed},
    sysvar::{rent::Rent, Sysvar},
};
use spl_token::state::Account as TokenAccount;
use thiserror::Error;

// Program ID
solana_program::declare_id!("LiquidityPool11111111111111111111111111111111");

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct PoolState {
    pub is_initialized: bool,
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub token_a_account: Pubkey,
    pub token_b_account: Pubkey,
    pub pool_mint: Pubkey,
    pub token_a_amount: u64,
    pub token_b_amount: u64,
    pub fee_rate: u64,  // Fee rate in basis points (1/10000)
}

#[derive(FromPrimitive, Debug)]
pub enum PoolInstruction {
    Initialize,
    AddLiquidity,
    RemoveLiquidity,
    Swap,
}

#[derive(Error, Debug, Copy, Clone)]
pub enum PoolError {
    #[error("Invalid instruction")]
    InvalidInstruction,
    #[error("Not rent exempt")]
    NotRentExempt,
    #[error("Expected amount mismatch")]
    ExpectedAmountMismatch,
    #[error("Pool already initialized")]
    AlreadyInUse,
    #[error("Invalid token account")]
    InvalidTokenAccount,
    #[error("Insufficient liquidity")]
    InsufficientLiquidity,
}

impl From<PoolError> for ProgramError {
    fn from(e: PoolError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = PoolInstruction::try_from_primitive(instruction_data[0])
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    match instruction {
        PoolInstruction::Initialize => {
            msg!("Instruction: Initialize Pool");
            process_initialize(program_id, accounts)
        }
        PoolInstruction::AddLiquidity => {
            msg!("Instruction: Add Liquidity");
            process_add_liquidity(program_id, accounts, &instruction_data[1..])
        }
        PoolInstruction::RemoveLiquidity => {
            msg!("Instruction: Remove Liquidity");
            process_remove_liquidity(program_id, accounts, &instruction_data[1..])
        }
        PoolInstruction::Swap => {
            msg!("Instruction: Swap");
            process_swap(program_id, accounts, &instruction_data[1..])
        }
    }
}

fn process_initialize(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let pool_account = next_account_info(account_info_iter)?;
    let token_a_mint = next_account_info(account_info_iter)?;
    let token_b_mint = next_account_info(account_info_iter)?;
    let pool_token_a = next_account_info(account_info_iter)?;
    let pool_token_b = next_account_info(account_info_iter)?;
    let pool_mint = next_account_info(account_info_iter)?;
    let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;

    // Verify account ownership and rent exemption
    if !rent.is_exempt(pool_account.lamports(), pool_account.data_len()) {
        return Err(PoolError::NotRentExempt.into());
    }

    let mut pool_state = PoolState::try_from_slice(&pool_account.data.borrow())?;
    if pool_state.is_initialized {
        return Err(PoolError::AlreadyInUse.into());
    }

    pool_state.is_initialized = true;
    pool_state.token_a_mint = *token_a_mint.key;
    pool_state.token_b_mint = *token_b_mint.key;
    pool_state.token_a_account = *pool_token_a.key;
    pool_state.token_b_account = *pool_token_b.key;
    pool_state.pool_mint = *pool_mint.key;
    pool_state.token_a_amount = 0;
    pool_state.token_b_amount = 0;
    pool_state.fee_rate = 30; // 0.3% fee

    pool_state.serialize(&mut *pool_account.data.borrow_mut())?;

    Ok(())
}

fn process_add_liquidity(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let pool_account = next_account_info(account_info_iter)?;
    let user_token_a = next_account_info(account_info_iter)?;
    let user_token_b = next_account_info(account_info_iter)?;
    let pool_token_a = next_account_info(account_info_iter)?;
    let pool_token_b = next_account_info(account_info_iter)?;
    let pool_mint = next_account_info(account_info_iter)?;
    let user_pool_token = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;

    let (amount_a, amount_b) = {
        let mut data = [0u8; 16];
        data[..8].copy_from_slice(&instruction_data[..8]);
        data[8..16].copy_from_slice(&instruction_data[8..16]);
        (u64::from_le_bytes(data[..8].try_into().unwrap()),
         u64::from_le_bytes(data[8..16].try_into().unwrap()))
    };

    let pool_state = PoolState::try_from_slice(&pool_account.data.borrow())?;

    // Calculate pool tokens to mint
    let pool_token_amount = if pool_state.token_a_amount == 0 {
        (amount_a as f64 * amount_b as f64).sqrt() as u64
    } else {
        std::cmp::min(
            amount_a * pool_state.token_b_amount / pool_state.token_a_amount,
            amount_b * pool_state.token_a_amount / pool_state.token_b_amount,
        )
    };

    // Transfer tokens to pool
    spl_token::instruction::transfer(
        token_program.key,
        user_token_a.key,
        pool_token_a.key,
        &pool_account.key,
        &[],
        amount_a,
    )?;

    spl_token::instruction::transfer(
        token_program.key,
        user_token_b.key,
        pool_token_b.key,
        &pool_account.key,
        &[],
        amount_b,
    )?;

    // Mint pool tokens to user
    spl_token::instruction::mint_to(
        token_program.key,
        pool_mint.key,
        user_pool_token.key,
        &pool_account.key,
        &[],
        pool_token_amount,
    )?;

    Ok(())
}

fn process_remove_liquidity(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let pool_account = next_account_info(account_info_iter)?;
    let user_token_a = next_account_info(account_info_iter)?;
    let user_token_b = next_account_info(account_info_iter)?;
    let pool_token_a = next_account_info(account_info_iter)?;
    let pool_token_b = next_account_info(account_info_iter)?;
    let user_pool_token = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;

    let pool_token_amount = {
        let mut data = [0u8; 8];
        data.copy_from_slice(&instruction_data[..8]);
        u64::from_le_bytes(data)
    };

    let pool_state = PoolState::try_from_slice(&pool_account.data.borrow())?;

    // Calculate token amounts to return
    let total_pool_tokens = spl_token::state::Mint::unpack(&pool_account.data.borrow())?.supply;
    let amount_a = pool_token_amount * pool_state.token_a_amount / total_pool_tokens;
    let amount_b = pool_token_amount * pool_state.token_b_amount / total_pool_tokens;

    // Transfer tokens from pool to user
    spl_token::instruction::transfer(
        token_program.key,
        pool_token_a.key,
        user_token_a.key,
        &pool_account.key,
        &[],
        amount_a,
    )?;

    spl_token::instruction::transfer(
        token_program.key,
        pool_token_b.key,
        user_token_b.key,
        &pool_account.key,
        &[],
        amount_b,
    )?;

    // Burn pool tokens
    spl_token::instruction::burn(
        token_program.key,
        user_pool_token.key,
        &pool_account.key,
        &pool_account.key,
        &[],
        pool_token_amount,
    )?;

    Ok(())
}

fn process_swap(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let pool_account = next_account_info(account_info_iter)?;
    let user_source = next_account_info(account_info_iter)?;
    let user_destination = next_account_info(account_info_iter)?;
    let pool_source = next_account_info(account_info_iter)?;
    let pool_destination = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;

    let amount_in = {
        let mut data = [0u8; 8];
        data.copy_from_slice(&instruction_data[..8]);
        u64::from_le_bytes(data)
    };

    let pool_state = PoolState::try_from_slice(&pool_account.data.borrow())?;

    // Calculate amount out using constant product formula
    let source_amount = TokenAccount::unpack(&pool_source.data.borrow())?.amount;
    let destination_amount = TokenAccount::unpack(&pool_destination.data.borrow())?.amount;
    
    let amount_out = calculate_output_amount(
        amount_in,
        source_amount,
        destination_amount,
        pool_state.fee_rate,
    )?;

    // Transfer tokens
    spl_token::instruction::transfer(
        token_program.key,
        user_source.key,
        pool_source.key,
        &pool_account.key,
        &[],
        amount_in,
    )?;

    spl_token::instruction::transfer(
        token_program.key,
        pool_destination.key,
        user_destination.key,
        &pool_account.key,
        &[],
        amount_out,
    )?;

    Ok(())
}

fn calculate_output_amount(
    amount_in: u64,
    reserve_in: u64,
    reserve_out: u64,
    fee_rate: u64,
) -> Result<u64, ProgramError> {
    let amount_in_with_fee = amount_in * (10000 - fee_rate);
    let numerator = amount_in_with_fee * reserve_out;
    let denominator = (reserve_in * 10000) + amount_in_with_fee;
    
    Ok((numerator / denominator) as u64)
}
