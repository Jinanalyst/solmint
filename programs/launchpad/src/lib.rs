use borsh::{BorshDeserialize, BorshSerialize};
use num_derive::FromPrimitive;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};
use spl_token::instruction as token_instruction;
use thiserror::Error;

// Program ID and Fee Wallet
solana_program::declare_id!("Launchpad11111111111111111111111111111111111");
pub const FEE_WALLET: &str = "6zkf4DviZZkpWVEh53MrcQV6vGXGpESnNXgAvU6KpBUH";

// Launchpad fees in lamports
pub const LAUNCH_BASE_FEE: u64 = 1_000_000_000;  // 1 SOL
pub const TIER_FEE: u64 = 500_000_000;          // 0.5 SOL per tier

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct LaunchpadConfig {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub total_supply: u64,
    pub tokens_for_presale: u64,
    pub price_per_token: u64,
    pub min_buy: u64,
    pub max_buy: u64,
    pub start_time: i64,
    pub end_time: i64,
    pub soft_cap: u64,
    pub hard_cap: u64,
    pub liquidity_percentage: u8,
    pub listing_price: u64,
    pub is_active: bool,
    pub total_sold: u64,
    pub total_raised: u64,
    pub tier_system: TierSystem,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct TierSystem {
    pub enabled: bool,
    pub tiers: Vec<Tier>,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Tier {
    pub name: String,
    pub required_tokens: u64,
    pub allocation_multiplier: u8,
    pub vesting_period: i64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Participant {
    pub wallet: Pubkey,
    pub amount_contributed: u64,
    pub tokens_owed: u64,
    pub tokens_claimed: u64,
    pub tier: u8,
    pub last_claim_time: i64,
}

#[derive(FromPrimitive, Debug)]
pub enum LaunchpadInstruction {
    CreateLaunchpad,
    ConfigureTiers,
    StartPresale,
    EndPresale,
    Participate,
    ClaimTokens,
    WithdrawFunds,
    CancelLaunch,
    AddToWhitelist,
    RemoveFromWhitelist,
}

#[derive(Error, Debug, Copy, Clone)]
pub enum LaunchpadError {
    #[error("Invalid instruction")]
    InvalidInstruction,
    #[error("Not rent exempt")]
    NotRentExempt,
    #[error("Invalid owner")]
    InvalidOwner,
    #[error("Presale not active")]
    PresaleNotActive,
    #[error("Presale ended")]
    PresaleEnded,
    #[error("Soft cap not reached")]
    SoftCapNotReached,
    #[error("Hard cap reached")]
    HardCapReached,
    #[error("Invalid amount")]
    InvalidAmount,
    #[error("Not whitelisted")]
    NotWhitelisted,
    #[error("Invalid tier")]
    InvalidTier,
    #[error("Vesting period not ended")]
    VestingPeriodNotEnded,
}

impl From<LaunchpadError> for ProgramError {
    fn from(e: LaunchpadError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = LaunchpadInstruction::try_from_primitive(instruction_data[0])
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    match instruction {
        LaunchpadInstruction::CreateLaunchpad => {
            process_create_launchpad(program_id, accounts, &instruction_data[1..])
        }
        LaunchpadInstruction::ConfigureTiers => {
            process_configure_tiers(program_id, accounts, &instruction_data[1..])
        }
        LaunchpadInstruction::StartPresale => {
            process_start_presale(program_id, accounts)
        }
        LaunchpadInstruction::EndPresale => {
            process_end_presale(program_id, accounts)
        }
        LaunchpadInstruction::Participate => {
            process_participate(program_id, accounts, &instruction_data[1..])
        }
        LaunchpadInstruction::ClaimTokens => {
            process_claim_tokens(program_id, accounts)
        }
        LaunchpadInstruction::WithdrawFunds => {
            process_withdraw_funds(program_id, accounts)
        }
        LaunchpadInstruction::CancelLaunch => {
            process_cancel_launch(program_id, accounts)
        }
        LaunchpadInstruction::AddToWhitelist => {
            process_add_to_whitelist(program_id, accounts, &instruction_data[1..])
        }
        LaunchpadInstruction::RemoveFromWhitelist => {
            process_remove_from_whitelist(program_id, accounts)
        }
    }
}

fn process_create_launchpad(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let owner_account = next_account_info(account_info_iter)?;
    let launchpad_account = next_account_info(account_info_iter)?;
    let mint_account = next_account_info(account_info_iter)?;
    let fee_wallet = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    // Verify fee wallet
    if fee_wallet.key.to_string() != FEE_WALLET {
        return Err(ProgramError::InvalidArgument);
    }

    let config = LaunchpadConfig::try_from_slice(instruction_data)?;
    let tier_count = if config.tier_system.enabled {
        config.tier_system.tiers.len() as u64
    } else {
        0
    };

    let total_fee = LAUNCH_BASE_FEE + (TIER_FEE * tier_count);

    // Transfer launch fee
    solana_program::program::invoke(
        &system_instruction::transfer(
            owner_account.key,
            fee_wallet.key,
            total_fee,
        ),
        &[
            owner_account.clone(),
            fee_wallet.clone(),
            system_program.clone(),
        ],
    )?;

    config.serialize(&mut *launchpad_account.data.borrow_mut())?;

    Ok(())
}

fn process_configure_tiers(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let owner_account = next_account_info(account_info_iter)?;
    let launchpad_account = next_account_info(account_info_iter)?;

    let mut config = LaunchpadConfig::try_from_slice(&launchpad_account.data.borrow())?;
    if config.owner != *owner_account.key {
        return Err(LaunchpadError::InvalidOwner.into());
    }

    let tier_system = TierSystem::try_from_slice(instruction_data)?;
    config.tier_system = tier_system;
    config.serialize(&mut *launchpad_account.data.borrow_mut())?;

    Ok(())
}

fn process_start_presale(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let owner_account = next_account_info(account_info_iter)?;
    let launchpad_account = next_account_info(account_info_iter)?;

    let mut config = LaunchpadConfig::try_from_slice(&launchpad_account.data.borrow())?;
    if config.owner != *owner_account.key {
        return Err(LaunchpadError::InvalidOwner.into());
    }

    config.is_active = true;
    config.start_time = solana_program::clock::Clock::get()?.unix_timestamp;
    config.serialize(&mut *launchpad_account.data.borrow_mut())?;

    Ok(())
}

fn process_participate(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let participant_account = next_account_info(account_info_iter)?;
    let launchpad_account = next_account_info(account_info_iter)?;
    let participant_info_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    let mut config = LaunchpadConfig::try_from_slice(&launchpad_account.data.borrow())?;
    if !config.is_active {
        return Err(LaunchpadError::PresaleNotActive.into());
    }

    let amount = u64::try_from_slice(instruction_data)?;
    if amount < config.min_buy || amount > config.max_buy {
        return Err(LaunchpadError::InvalidAmount.into());
    }

    if config.total_raised.checked_add(amount).unwrap() > config.hard_cap {
        return Err(LaunchpadError::HardCapReached.into());
    }

    // Transfer SOL to launchpad account
    solana_program::program::invoke(
        &system_instruction::transfer(
            participant_account.key,
            launchpad_account.key,
            amount,
        ),
        &[
            participant_account.clone(),
            launchpad_account.clone(),
            system_program.clone(),
        ],
    )?;

    let tokens_amount = amount
        .checked_mul(config.tokens_for_presale)
        .unwrap()
        .checked_div(config.hard_cap)
        .unwrap();

    let mut participant_info = if participant_info_account.data_is_empty() {
        Participant {
            wallet: *participant_account.key,
            amount_contributed: amount,
            tokens_owed: tokens_amount,
            tokens_claimed: 0,
            tier: 0,
            last_claim_time: 0,
        }
    } else {
        let mut info = Participant::try_from_slice(&participant_info_account.data.borrow())?;
        info.amount_contributed = info.amount_contributed.checked_add(amount).unwrap();
        info.tokens_owed = info.tokens_owed.checked_add(tokens_amount).unwrap();
        info
    };

    participant_info.serialize(&mut *participant_info_account.data.borrow_mut())?;

    config.total_raised = config.total_raised.checked_add(amount).unwrap();
    config.total_sold = config.total_sold.checked_add(tokens_amount).unwrap();
    config.serialize(&mut *launchpad_account.data.borrow_mut())?;

    Ok(())
}

fn process_claim_tokens(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let participant_account = next_account_info(account_info_iter)?;
    let launchpad_account = next_account_info(account_info_iter)?;
    let participant_info_account = next_account_info(account_info_iter)?;
    let token_account = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;

    let config = LaunchpadConfig::try_from_slice(&launchpad_account.data.borrow())?;
    let mut participant_info = Participant::try_from_slice(&participant_info_account.data.borrow())?;

    if config.is_active {
        return Err(LaunchpadError::PresaleNotActive.into());
    }

    if config.total_raised < config.soft_cap {
        return Err(LaunchpadError::SoftCapNotReached.into());
    }

    let current_time = solana_program::clock::Clock::get()?.unix_timestamp;
    if config.tier_system.enabled {
        let tier = &config.tier_system.tiers[participant_info.tier as usize];
        if current_time < participant_info.last_claim_time + tier.vesting_period {
            return Err(LaunchpadError::VestingPeriodNotEnded.into());
        }
    }

    let claimable_amount = participant_info.tokens_owed
        .checked_sub(participant_info.tokens_claimed)
        .unwrap();

    // Transfer tokens
    solana_program::program::invoke(
        &token_instruction::transfer(
            token_program.key,
            token_account.key,
            participant_account.key,
            &config.owner,
            &[&config.owner],
            claimable_amount,
        )?,
        &[
            token_account.clone(),
            participant_account.clone(),
            owner_account.clone(),
            token_program.clone(),
        ],
    )?;

    participant_info.tokens_claimed = participant_info.tokens_claimed
        .checked_add(claimable_amount)
        .unwrap();
    participant_info.last_claim_time = current_time;
    participant_info.serialize(&mut *participant_info_account.data.borrow_mut())?;

    Ok(())
}

fn process_withdraw_funds(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let owner_account = next_account_info(account_info_iter)?;
    let launchpad_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    let config = LaunchpadConfig::try_from_slice(&launchpad_account.data.borrow())?;
    if config.owner != *owner_account.key {
        return Err(LaunchpadError::InvalidOwner.into());
    }

    if config.is_active {
        return Err(LaunchpadError::PresaleNotActive.into());
    }

    if config.total_raised < config.soft_cap {
        return Err(LaunchpadError::SoftCapNotReached.into());
    }

    let lamports = launchpad_account.lamports();
    **launchpad_account.lamports.borrow_mut() = 0;
    **owner_account.lamports.borrow_mut() = owner_account
        .lamports()
        .checked_add(lamports)
        .unwrap();

    Ok(())
}

fn process_cancel_launch(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let owner_account = next_account_info(account_info_iter)?;
    let launchpad_account = next_account_info(account_info_iter)?;

    let mut config = LaunchpadConfig::try_from_slice(&launchpad_account.data.borrow())?;
    if config.owner != *owner_account.key {
        return Err(LaunchpadError::InvalidOwner.into());
    }

    config.is_active = false;
    config.end_time = solana_program::clock::Clock::get()?.unix_timestamp;
    config.serialize(&mut *launchpad_account.data.borrow_mut())?;

    Ok(())
}

fn process_add_to_whitelist(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let owner_account = next_account_info(account_info_iter)?;
    let launchpad_account = next_account_info(account_info_iter)?;
    let whitelist_account = next_account_info(account_info_iter)?;

    let config = LaunchpadConfig::try_from_slice(&launchpad_account.data.borrow())?;
    if config.owner != *owner_account.key {
        return Err(LaunchpadError::InvalidOwner.into());
    }

    let wallet = Pubkey::try_from_slice(instruction_data)?;
    wallet.serialize(&mut *whitelist_account.data.borrow_mut())?;

    Ok(())
}

fn process_remove_from_whitelist(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let owner_account = next_account_info(account_info_iter)?;
    let launchpad_account = next_account_info(account_info_iter)?;
    let whitelist_account = next_account_info(account_info_iter)?;

    let config = LaunchpadConfig::try_from_slice(&launchpad_account.data.borrow())?;
    if config.owner != *owner_account.key {
        return Err(LaunchpadError::InvalidOwner.into());
    }

    // Close whitelist account
    let dest_starting_lamports = owner_account.lamports();
    **owner_account.lamports.borrow_mut() = dest_starting_lamports
        .checked_add(whitelist_account.lamports())
        .unwrap();
    **whitelist_account.lamports.borrow_mut() = 0;

    Ok(())
}
