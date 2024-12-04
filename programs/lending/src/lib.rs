use borsh::{BorshDeserialize, BorshSerialize};
use num_derive::FromPrimitive;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    clock::Clock,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::Sysvar,
};
use spl_token::state::Account as TokenAccount;
use thiserror::Error;

// Program ID and Fee Wallet
solana_program::declare_id!("LendingPool11111111111111111111111111111111");
pub const FEE_WALLET: &str = "6zkf4DviZZkpWVEh53MrcQV6vGXGpESnNXgAvU6KpBUH";
pub const SERVICE_FEE_BPS: u64 = 20; // 0.2% fee for lending operations

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct LendingPool {
    pub is_initialized: bool,
    pub token_mint: Pubkey,
    pub pool_authority: Pubkey,
    pub lending_token_account: Pubkey,
    pub total_deposits: u64,
    pub total_borrows: u64,
    pub last_update_time: i64,
    pub lending_rate: u64,     // Lending interest rate (basis points)
    pub borrowing_rate: u64,   // Borrowing interest rate (basis points)
    pub collateral_ratio: u64, // Required collateral ratio (percentage * 100)
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct UserLendingInfo {
    pub owner: Pubkey,
    pub deposited_amount: u64,
    pub borrowed_amount: u64,
    pub collateral_amount: u64,
    pub last_update_time: i64,
    pub cumulative_deposit_interest: u64,
    pub cumulative_borrow_interest: u64,
}

#[derive(FromPrimitive, Debug)]
pub enum LendingInstruction {
    Initialize,
    Deposit,
    Withdraw,
    Borrow,
    Repay,
    AddCollateral,
    WithdrawCollateral,
    LiquidatePosition,
}

#[derive(Error, Debug, Copy, Clone)]
pub enum LendingError {
    #[error("Invalid instruction")]
    InvalidInstruction,
    #[error("Not rent exempt")]
    NotRentExempt,
    #[error("Pool already initialized")]
    AlreadyInUse,
    #[error("Invalid token account")]
    InvalidTokenAccount,
    #[error("Insufficient collateral")]
    InsufficientCollateral,
    #[error("Insufficient liquidity")]
    InsufficientLiquidity,
    #[error("Position not liquidatable")]
    PositionNotLiquidatable,
}

impl From<LendingError> for ProgramError {
    fn from(e: LendingError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = LendingInstruction::try_from_primitive(instruction_data[0])
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    match instruction {
        LendingInstruction::Initialize => {
            msg!("Instruction: Initialize Lending Pool");
            process_initialize(program_id, accounts)
        }
        LendingInstruction::Deposit => {
            msg!("Instruction: Deposit Tokens");
            process_deposit(program_id, accounts, &instruction_data[1..])
        }
        LendingInstruction::Withdraw => {
            msg!("Instruction: Withdraw Tokens");
            process_withdraw(program_id, accounts, &instruction_data[1..])
        }
        LendingInstruction::Borrow => {
            msg!("Instruction: Borrow Tokens");
            process_borrow(program_id, accounts, &instruction_data[1..])
        }
        LendingInstruction::Repay => {
            msg!("Instruction: Repay Loan");
            process_repay(program_id, accounts, &instruction_data[1..])
        }
        LendingInstruction::AddCollateral => {
            msg!("Instruction: Add Collateral");
            process_add_collateral(program_id, accounts, &instruction_data[1..])
        }
        LendingInstruction::WithdrawCollateral => {
            msg!("Instruction: Withdraw Collateral");
            process_withdraw_collateral(program_id, accounts, &instruction_data[1..])
        }
        LendingInstruction::LiquidatePosition => {
            msg!("Instruction: Liquidate Position");
            process_liquidate_position(program_id, accounts, &instruction_data[1..])
        }
    }
}

fn process_initialize(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let pool_account = next_account_info(account_info_iter)?;
    let token_mint = next_account_info(account_info_iter)?;
    let pool_authority = next_account_info(account_info_iter)?;
    let lending_token_account = next_account_info(account_info_iter)?;

    let mut pool = LendingPool::try_from_slice(&pool_account.data.borrow())?;
    if pool.is_initialized {
        return Err(LendingError::AlreadyInUse.into());
    }

    pool.is_initialized = true;
    pool.token_mint = *token_mint.key;
    pool.pool_authority = *pool_authority.key;
    pool.lending_token_account = *lending_token_account.key;
    pool.total_deposits = 0;
    pool.total_borrows = 0;
    pool.last_update_time = Clock::get()?.unix_timestamp;
    pool.lending_rate = 500;     // 5% APY
    pool.borrowing_rate = 1000;  // 10% APR
    pool.collateral_ratio = 15000; // 150%

    pool.serialize(&mut *pool_account.data.borrow_mut())?;

    Ok(())
}

fn process_deposit(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let pool_account = next_account_info(account_info_iter)?;
    let user_lending_info = next_account_info(account_info_iter)?;
    let user_token_account = next_account_info(account_info_iter)?;
    let pool_token_account = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;
    let clock = Clock::get()?;

    let amount = {
        let mut data = [0u8; 8];
        data.copy_from_slice(&instruction_data[..8]);
        u64::from_le_bytes(data)
    };

    let mut pool = LendingPool::try_from_slice(&pool_account.data.borrow())?;
    let mut user_info = if user_lending_info.data_len() > 0 {
        UserLendingInfo::try_from_slice(&user_lending_info.data.borrow())?
    } else {
        UserLendingInfo {
            owner: *user_token_account.key,
            deposited_amount: 0,
            borrowed_amount: 0,
            collateral_amount: 0,
            last_update_time: clock.unix_timestamp,
            cumulative_deposit_interest: 0,
            cumulative_borrow_interest: 0,
        }
    };

    // Update interest before deposit
    update_interest(&mut pool, &mut user_info, clock.unix_timestamp)?;

    // Transfer tokens to pool
    spl_token::instruction::transfer(
        token_program.key,
        user_token_account.key,
        pool_token_account.key,
        &user_token_account.key,
        &[],
        amount,
    )?;

    user_info.deposited_amount = user_info.deposited_amount.checked_add(amount)
        .ok_or(ProgramError::Overflow)?;
    pool.total_deposits = pool.total_deposits.checked_add(amount)
        .ok_or(ProgramError::Overflow)?;

    pool.serialize(&mut *pool_account.data.borrow_mut())?;
    user_info.serialize(&mut *user_lending_info.data.borrow_mut())?;

    Ok(())
}

fn process_withdraw(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let pool_account = next_account_info(account_info_iter)?;
    let user_lending_info = next_account_info(account_info_iter)?;
    let user_token_account = next_account_info(account_info_iter)?;
    let pool_token_account = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;
    let clock = Clock::get()?;

    let amount = {
        let mut data = [0u8; 8];
        data.copy_from_slice(&instruction_data[..8]);
        u64::from_le_bytes(data)
    };

    let mut pool = LendingPool::try_from_slice(&pool_account.data.borrow())?;
    let mut user_info = UserLendingInfo::try_from_slice(&user_lending_info.data.borrow())?;

    // Update interest before withdrawal
    update_interest(&mut pool, &mut user_info, clock.unix_timestamp)?;

    // Check if user has enough available balance
    if amount > user_info.deposited_amount {
        return Err(LendingError::InsufficientLiquidity.into());
    }

    // Check collateral ratio after withdrawal
    let remaining_deposit = user_info.deposited_amount.checked_sub(amount)
        .ok_or(ProgramError::Overflow)?;
    if !check_collateral_ratio(&pool, remaining_deposit, user_info.borrowed_amount) {
        return Err(LendingError::InsufficientCollateral.into());
    }

    // Transfer tokens back to user
    spl_token::instruction::transfer(
        token_program.key,
        pool_token_account.key,
        user_token_account.key,
        &pool_account.key,
        &[],
        amount,
    )?;

    user_info.deposited_amount = remaining_deposit;
    pool.total_deposits = pool.total_deposits.checked_sub(amount)
        .ok_or(ProgramError::Overflow)?;

    pool.serialize(&mut *pool_account.data.borrow_mut())?;
    user_info.serialize(&mut *user_lending_info.data.borrow_mut())?;

    Ok(())
}

fn process_borrow(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let pool_account = next_account_info(account_info_iter)?;
    let user_lending_info = next_account_info(account_info_iter)?;
    let user_token_account = next_account_info(account_info_iter)?;
    let pool_token_account = next_account_info(account_info_iter)?;
    let fee_wallet_account = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;
    let clock = Clock::get()?;

    // Verify fee wallet
    if fee_wallet_account.key.to_string() != FEE_WALLET {
        return Err(ProgramError::InvalidArgument);
    }

    let amount = {
        let mut data = [0u8; 8];
        data.copy_from_slice(&instruction_data[..8]);
        u64::from_le_bytes(data)
    };

    let mut pool = LendingPool::try_from_slice(&pool_account.data.borrow())?;
    let mut user_info = UserLendingInfo::try_from_slice(&user_lending_info.data.borrow())?;

    // Update interest before borrowing
    update_interest(&mut pool, &mut user_info, clock.unix_timestamp)?;

    // Check if pool has enough liquidity
    if amount > pool.total_deposits.checked_sub(pool.total_borrows)
        .ok_or(ProgramError::Overflow)? {
        return Err(LendingError::InsufficientLiquidity.into());
    }

    // Check if user has enough collateral
    let new_borrow_amount = user_info.borrowed_amount.checked_add(amount)
        .ok_or(ProgramError::Overflow)?;
    if !check_collateral_ratio(&pool, user_info.deposited_amount, new_borrow_amount) {
        return Err(LendingError::InsufficientCollateral.into());
    }

    // Calculate service fee
    let fee_amount = amount
        .checked_mul(SERVICE_FEE_BPS)
        .ok_or(ProgramError::Overflow)?
        .checked_div(10000)
        .ok_or(ProgramError::Overflow)?;
    let user_borrow_amount = amount.checked_sub(fee_amount)
        .ok_or(ProgramError::Overflow)?;

    // Transfer tokens to user
    spl_token::instruction::transfer(
        token_program.key,
        pool_token_account.key,
        user_token_account.key,
        &pool_account.key,
        &[],
        user_borrow_amount,
    )?;

    // Transfer fee to fee wallet
    spl_token::instruction::transfer(
        token_program.key,
        pool_token_account.key,
        fee_wallet_account.key,
        &pool_account.key,
        &[],
        fee_amount,
    )?;

    user_info.borrowed_amount = new_borrow_amount;
    pool.total_borrows = pool.total_borrows.checked_add(amount)
        .ok_or(ProgramError::Overflow)?;

    pool.serialize(&mut *pool_account.data.borrow_mut())?;
    user_info.serialize(&mut *user_lending_info.data.borrow_mut())?;

    Ok(())
}

fn process_repay(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let pool_account = next_account_info(account_info_iter)?;
    let user_lending_info = next_account_info(account_info_iter)?;
    let user_token_account = next_account_info(account_info_iter)?;
    let pool_token_account = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;
    let clock = Clock::get()?;

    let amount = {
        let mut data = [0u8; 8];
        data.copy_from_slice(&instruction_data[..8]);
        u64::from_le_bytes(data)
    };

    let mut pool = LendingPool::try_from_slice(&pool_account.data.borrow())?;
    let mut user_info = UserLendingInfo::try_from_slice(&user_lending_info.data.borrow())?;

    // Update interest before repayment
    update_interest(&mut pool, &mut user_info, clock.unix_timestamp)?;

    let repay_amount = std::cmp::min(amount, user_info.borrowed_amount);

    // Transfer tokens to pool
    spl_token::instruction::transfer(
        token_program.key,
        user_token_account.key,
        pool_token_account.key,
        &user_token_account.key,
        &[],
        repay_amount,
    )?;

    user_info.borrowed_amount = user_info.borrowed_amount.checked_sub(repay_amount)
        .ok_or(ProgramError::Overflow)?;
    pool.total_borrows = pool.total_borrows.checked_sub(repay_amount)
        .ok_or(ProgramError::Overflow)?;

    pool.serialize(&mut *pool_account.data.borrow_mut())?;
    user_info.serialize(&mut *user_lending_info.data.borrow_mut())?;

    Ok(())
}

fn update_interest(
    pool: &mut LendingPool,
    user: &mut UserLendingInfo,
    current_time: i64,
) -> ProgramResult {
    let time_elapsed = (current_time - user.last_update_time) as u64;
    if time_elapsed > 0 {
        // Calculate deposit interest
        if user.deposited_amount > 0 {
            let deposit_interest = user.deposited_amount
                .checked_mul(pool.lending_rate)
                .ok_or(ProgramError::Overflow)?
                .checked_mul(time_elapsed)
                .ok_or(ProgramError::Overflow)?
                .checked_div(365 * 24 * 60 * 60 * 10000)
                .ok_or(ProgramError::Overflow)?;

            user.cumulative_deposit_interest = user.cumulative_deposit_interest
                .checked_add(deposit_interest)
                .ok_or(ProgramError::Overflow)?;
        }

        // Calculate borrow interest
        if user.borrowed_amount > 0 {
            let borrow_interest = user.borrowed_amount
                .checked_mul(pool.borrowing_rate)
                .ok_or(ProgramError::Overflow)?
                .checked_mul(time_elapsed)
                .ok_or(ProgramError::Overflow)?
                .checked_div(365 * 24 * 60 * 60 * 10000)
                .ok_or(ProgramError::Overflow)?;

            user.cumulative_borrow_interest = user.cumulative_borrow_interest
                .checked_add(borrow_interest)
                .ok_or(ProgramError::Overflow)?;
        }

        user.last_update_time = current_time;
    }

    Ok(())
}

fn check_collateral_ratio(
    pool: &LendingPool,
    deposit_amount: u64,
    borrow_amount: u64,
) -> bool {
    if borrow_amount == 0 {
        return true;
    }

    let collateral_value = (deposit_amount as u128)
        .checked_mul(10000)
        .unwrap_or(0);
    let required_collateral = (borrow_amount as u128)
        .checked_mul(pool.collateral_ratio as u128)
        .unwrap_or(0);

    collateral_value >= required_collateral
}

fn process_add_collateral(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Similar to deposit but updates collateral amount
    Ok(())
}

fn process_withdraw_collateral(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Similar to withdraw but checks collateral ratio
    Ok(())
}

fn process_liquidate_position(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Implement liquidation logic for undercollateralized positions
    Ok(())
}
