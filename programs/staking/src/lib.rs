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
solana_program::declare_id!("StakingPool111111111111111111111111111111111");
pub const FEE_WALLET: &str = "6zkf4DviZZkpWVEh53MrcQV6vGXGpESnNXgAvU6KpBUH";
pub const SERVICE_FEE_BPS: u64 = 30; // 0.3% fee

// Program ID
// solana_program::declare_id!("StakingPool111111111111111111111111111111111");

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct StakePool {
    pub is_initialized: bool,
    pub token_mint: Pubkey,
    pub pool_authority: Pubkey,
    pub stake_token_account: Pubkey,
    pub reward_token_account: Pubkey,
    pub total_staked: u64,
    pub reward_rate: u64,  // Rewards per second
    pub last_update_time: i64,
    pub reward_per_token_stored: u128,
}

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct UserStakeInfo {
    pub owner: Pubkey,
    pub stake_amount: u64,
    pub rewards_earned: u64,
    pub reward_per_token_paid: u128,
    pub start_time: i64,
    pub lock_period: i64,  // Lock period in seconds
}

#[derive(FromPrimitive, Debug)]
pub enum StakingInstruction {
    Initialize,
    Stake,
    Unstake,
    ClaimReward,
    UpdatePool,
}

#[derive(Error, Debug, Copy, Clone)]
pub enum StakingError {
    #[error("Invalid instruction")]
    InvalidInstruction,
    #[error("Not rent exempt")]
    NotRentExempt,
    #[error("Pool already initialized")]
    AlreadyInUse,
    #[error("Invalid token account")]
    InvalidTokenAccount,
    #[error("Insufficient stake balance")]
    InsufficientStakeBalance,
    #[error("Stake still locked")]
    StakeLocked,
}

impl From<StakingError> for ProgramError {
    fn from(e: StakingError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = StakingInstruction::try_from_primitive(instruction_data[0])
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    match instruction {
        StakingInstruction::Initialize => {
            msg!("Instruction: Initialize Staking Pool");
            process_initialize(program_id, accounts)
        }
        StakingInstruction::Stake => {
            msg!("Instruction: Stake Tokens");
            process_stake(program_id, accounts, &instruction_data[1..])
        }
        StakingInstruction::Unstake => {
            msg!("Instruction: Unstake Tokens");
            process_unstake(program_id, accounts, &instruction_data[1..])
        }
        StakingInstruction::ClaimReward => {
            msg!("Instruction: Claim Reward");
            process_claim_reward(program_id, accounts)
        }
        StakingInstruction::UpdatePool => {
            msg!("Instruction: Update Pool");
            process_update_pool(program_id, accounts)
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
    let stake_token_account = next_account_info(account_info_iter)?;
    let reward_token_account = next_account_info(account_info_iter)?;

    let mut pool = StakePool::try_from_slice(&pool_account.data.borrow())?;
    if pool.is_initialized {
        return Err(StakingError::AlreadyInUse.into());
    }

    pool.is_initialized = true;
    pool.token_mint = *token_mint.key;
    pool.pool_authority = *pool_authority.key;
    pool.stake_token_account = *stake_token_account.key;
    pool.reward_token_account = *reward_token_account.key;
    pool.total_staked = 0;
    pool.reward_rate = 100; // Example: 100 tokens per second
    pool.last_update_time = Clock::get()?.unix_timestamp;
    pool.reward_per_token_stored = 0;

    pool.serialize(&mut *pool_account.data.borrow_mut())?;

    Ok(())
}

fn process_stake(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let pool_account = next_account_info(account_info_iter)?;
    let user_stake_info = next_account_info(account_info_iter)?;
    let user_token_account = next_account_info(account_info_iter)?;
    let pool_token_account = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;
    let clock = Clock::get()?;

    let amount = {
        let mut data = [0u8; 8];
        data.copy_from_slice(&instruction_data[..8]);
        u64::from_le_bytes(data)
    };

    let mut pool = StakePool::try_from_slice(&pool_account.data.borrow())?;
    let mut user_info = if user_stake_info.data_len() > 0 {
        UserStakeInfo::try_from_slice(&user_stake_info.data.borrow())?
    } else {
        UserStakeInfo {
            owner: *user_token_account.key,
            stake_amount: 0,
            rewards_earned: 0,
            reward_per_token_paid: 0,
            start_time: clock.unix_timestamp,
            lock_period: 7 * 24 * 60 * 60, // 7 days lock period
        }
    };

    // Update pool and calculate rewards before stake
    update_pool(&mut pool, clock.unix_timestamp)?;
    update_rewards(&mut pool, &mut user_info)?;

    // Transfer tokens to pool
    spl_token::instruction::transfer(
        token_program.key,
        user_token_account.key,
        pool_token_account.key,
        &user_token_account.key,
        &[],
        amount,
    )?;

    user_info.stake_amount = user_info.stake_amount.checked_add(amount)
        .ok_or(ProgramError::Overflow)?;
    pool.total_staked = pool.total_staked.checked_add(amount)
        .ok_or(ProgramError::Overflow)?;

    pool.serialize(&mut *pool_account.data.borrow_mut())?;
    user_info.serialize(&mut *user_stake_info.data.borrow_mut())?;

    Ok(())
}

fn process_unstake(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let pool_account = next_account_info(account_info_iter)?;
    let user_stake_info = next_account_info(account_info_iter)?;
    let user_token_account = next_account_info(account_info_iter)?;
    let pool_token_account = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;
    let clock = Clock::get()?;

    let amount = {
        let mut data = [0u8; 8];
        data.copy_from_slice(&instruction_data[..8]);
        u64::from_le_bytes(data)
    };

    let mut pool = StakePool::try_from_slice(&pool_account.data.borrow())?;
    let mut user_info = UserStakeInfo::try_from_slice(&user_stake_info.data.borrow())?;

    // Check lock period
    if clock.unix_timestamp < user_info.start_time + user_info.lock_period {
        return Err(StakingError::StakeLocked.into());
    }

    if amount > user_info.stake_amount {
        return Err(StakingError::InsufficientStakeBalance.into());
    }

    // Update pool and calculate rewards before unstake
    update_pool(&mut pool, clock.unix_timestamp)?;
    update_rewards(&mut pool, &mut user_info)?;

    // Transfer tokens back to user
    spl_token::instruction::transfer(
        token_program.key,
        pool_token_account.key,
        user_token_account.key,
        &pool_account.key,
        &[],
        amount,
    )?;

    user_info.stake_amount = user_info.stake_amount.checked_sub(amount)
        .ok_or(ProgramError::Overflow)?;
    pool.total_staked = pool.total_staked.checked_sub(amount)
        .ok_or(ProgramError::Overflow)?;

    pool.serialize(&mut *pool_account.data.borrow_mut())?;
    user_info.serialize(&mut *user_stake_info.data.borrow_mut())?;

    Ok(())
}

fn process_claim_reward(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let pool_account = next_account_info(account_info_iter)?;
    let user_stake_info = next_account_info(account_info_iter)?;
    let user_reward_account = next_account_info(account_info_iter)?;
    let pool_reward_account = next_account_info(account_info_iter)?;
    let fee_wallet_account = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;
    let clock = Clock::get()?;

    // Verify fee wallet
    if fee_wallet_account.key.to_string() != FEE_WALLET {
        return Err(ProgramError::InvalidArgument);
    }

    let mut pool = StakePool::try_from_slice(&pool_account.data.borrow())?;
    let mut user_info = UserStakeInfo::try_from_slice(&user_stake_info.data.borrow())?;

    // Update rewards
    update_pool(&mut pool, clock.unix_timestamp)?;
    update_rewards(&mut pool, &mut user_info)?;

    let reward_amount = user_info.rewards_earned;
    if reward_amount > 0 {
        // Calculate service fee
        let fee_amount = reward_amount
            .checked_mul(SERVICE_FEE_BPS)
            .ok_or(ProgramError::Overflow)?
            .checked_div(10000)
            .ok_or(ProgramError::Overflow)?;
        let user_reward = reward_amount.checked_sub(fee_amount)
            .ok_or(ProgramError::Overflow)?;

        // Transfer rewards to user
        spl_token::instruction::transfer(
            token_program.key,
            pool_reward_account.key,
            user_reward_account.key,
            &pool_account.key,
            &[],
            user_reward,
        )?;

        // Transfer fee to fee wallet
        spl_token::instruction::transfer(
            token_program.key,
            pool_reward_account.key,
            fee_wallet_account.key,
            &pool_account.key,
            &[],
            fee_amount,
        )?;

        user_info.rewards_earned = 0;
    }

    pool.serialize(&mut *pool_account.data.borrow_mut())?;
    user_info.serialize(&mut *user_stake_info.data.borrow_mut())?;

    Ok(())
}

fn process_update_pool(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let pool_account = next_account_info(account_info_iter)?;
    let clock = Clock::get()?;

    let mut pool = StakePool::try_from_slice(&pool_account.data.borrow())?;
    update_pool(&mut pool, clock.unix_timestamp)?;
    pool.serialize(&mut *pool_account.data.borrow_mut())?;

    Ok(())
}

fn update_pool(
    pool: &mut StakePool,
    current_time: i64,
) -> ProgramResult {
    if pool.total_staked == 0 {
        pool.last_update_time = current_time;
        return Ok(());
    }

    let time_elapsed = current_time - pool.last_update_time;
    if time_elapsed > 0 {
        let reward = (time_elapsed as u64).checked_mul(pool.reward_rate)
            .ok_or(ProgramError::Overflow)?;
        let reward_per_token = (reward as u128)
            .checked_mul(1_000_000_000_000u128)
            .ok_or(ProgramError::Overflow)?
            .checked_div(pool.total_staked as u128)
            .ok_or(ProgramError::Overflow)?;
        
        pool.reward_per_token_stored = pool.reward_per_token_stored
            .checked_add(reward_per_token)
            .ok_or(ProgramError::Overflow)?;
        pool.last_update_time = current_time;
    }

    Ok(())
}

fn update_rewards(
    pool: &StakePool,
    user: &mut UserStakeInfo,
) -> ProgramResult {
    let reward_per_token = pool.reward_per_token_stored;
    let rewards = (user.stake_amount as u128)
        .checked_mul(reward_per_token.checked_sub(user.reward_per_token_paid)
            .ok_or(ProgramError::Overflow)?)
        .ok_or(ProgramError::Overflow)?
        .checked_div(1_000_000_000_000u128)
        .ok_or(ProgramError::Overflow)?;

    user.rewards_earned = user.rewards_earned
        .checked_add(rewards as u64)
        .ok_or(ProgramError::Overflow)?;
    user.reward_per_token_paid = reward_per_token;

    Ok(())
}
