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
solana_program::declare_id!("Airdrop1111111111111111111111111111111111111");
pub const FEE_WALLET: &str = "6zkf4DviZZkpWVEh53MrcQV6vGXGpESnNXgAvU6KpBUH";

// Airdrop fees in lamports
pub const AIRDROP_BASE_FEE: u64 = 100_000_000;  // 0.1 SOL
pub const PER_RECIPIENT_FEE: u64 = 1_000_000;   // 0.001 SOL

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct AirdropCampaign {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub total_amount: u64,
    pub amount_per_recipient: u64,
    pub start_time: i64,
    pub end_time: i64,
    pub is_active: bool,
    pub claimed_count: u64,
    pub max_recipients: u64,
    pub whitelist_required: bool,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct WhitelistEntry {
    pub wallet: Pubkey,
    pub has_claimed: bool,
}

#[derive(FromPrimitive, Debug)]
pub enum AirdropInstruction {
    CreateCampaign,
    AddToWhitelist,
    RemoveFromWhitelist,
    StartAirdrop,
    EndAirdrop,
    ClaimAirdrop,
    WithdrawRemainingTokens,
}

#[derive(Error, Debug, Copy, Clone)]
pub enum AirdropError {
    #[error("Invalid instruction")]
    InvalidInstruction,
    #[error("Not rent exempt")]
    NotRentExempt,
    #[error("Campaign already exists")]
    CampaignAlreadyExists,
    #[error("Invalid campaign owner")]
    InvalidCampaignOwner,
    #[error("Campaign not active")]
    CampaignNotActive,
    #[error("Campaign ended")]
    CampaignEnded,
    #[error("Not whitelisted")]
    NotWhitelisted,
    #[error("Already claimed")]
    AlreadyClaimed,
    #[error("Maximum recipients reached")]
    MaxRecipientsReached,
    #[error("Insufficient funds")]
    InsufficientFunds,
}

impl From<AirdropError> for ProgramError {
    fn from(e: AirdropError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = AirdropInstruction::try_from_primitive(instruction_data[0])
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    match instruction {
        AirdropInstruction::CreateCampaign => {
            process_create_campaign(program_id, accounts, &instruction_data[1..])
        }
        AirdropInstruction::AddToWhitelist => {
            process_add_to_whitelist(program_id, accounts, &instruction_data[1..])
        }
        AirdropInstruction::RemoveFromWhitelist => {
            process_remove_from_whitelist(program_id, accounts, &instruction_data[1..])
        }
        AirdropInstruction::StartAirdrop => {
            process_start_airdrop(program_id, accounts)
        }
        AirdropInstruction::EndAirdrop => {
            process_end_airdrop(program_id, accounts)
        }
        AirdropInstruction::ClaimAirdrop => {
            process_claim_airdrop(program_id, accounts)
        }
        AirdropInstruction::WithdrawRemainingTokens => {
            process_withdraw_remaining_tokens(program_id, accounts)
        }
    }
}

fn process_create_campaign(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let owner_account = next_account_info(account_info_iter)?;
    let campaign_account = next_account_info(account_info_iter)?;
    let mint_account = next_account_info(account_info_iter)?;
    let fee_wallet = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;

    // Verify fee wallet
    if fee_wallet.key.to_string() != FEE_WALLET {
        return Err(ProgramError::InvalidArgument);
    }

    let campaign_data = AirdropCampaign::try_from_slice(instruction_data)?;
    let total_fee = AIRDROP_BASE_FEE + (PER_RECIPIENT_FEE * campaign_data.max_recipients);

    // Transfer airdrop fee
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

    campaign_data.serialize(&mut *campaign_account.data.borrow_mut())?;

    Ok(())
}

fn process_add_to_whitelist(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let owner_account = next_account_info(account_info_iter)?;
    let campaign_account = next_account_info(account_info_iter)?;
    let whitelist_account = next_account_info(account_info_iter)?;

    let campaign_data = AirdropCampaign::try_from_slice(&campaign_account.data.borrow())?;
    if campaign_data.owner != *owner_account.key {
        return Err(AirdropError::InvalidCampaignOwner.into());
    }

    let whitelist_entry = WhitelistEntry::try_from_slice(instruction_data)?;
    whitelist_entry.serialize(&mut *whitelist_account.data.borrow_mut())?;

    Ok(())
}

fn process_remove_from_whitelist(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let owner_account = next_account_info(account_info_iter)?;
    let campaign_account = next_account_info(account_info_iter)?;
    let whitelist_account = next_account_info(account_info_iter)?;

    let campaign_data = AirdropCampaign::try_from_slice(&campaign_account.data.borrow())?;
    if campaign_data.owner != *owner_account.key {
        return Err(AirdropError::InvalidCampaignOwner.into());
    }

    // Close whitelist account
    let dest_starting_lamports = owner_account.lamports();
    **owner_account.lamports.borrow_mut() = dest_starting_lamports
        .checked_add(whitelist_account.lamports())
        .unwrap();
    **whitelist_account.lamports.borrow_mut() = 0;

    Ok(())
}

fn process_start_airdrop(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let owner_account = next_account_info(account_info_iter)?;
    let campaign_account = next_account_info(account_info_iter)?;

    let mut campaign_data = AirdropCampaign::try_from_slice(&campaign_account.data.borrow())?;
    if campaign_data.owner != *owner_account.key {
        return Err(AirdropError::InvalidCampaignOwner.into());
    }

    campaign_data.is_active = true;
    campaign_data.start_time = solana_program::clock::Clock::get()?.unix_timestamp;
    campaign_data.serialize(&mut *campaign_account.data.borrow_mut())?;

    Ok(())
}

fn process_end_airdrop(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let owner_account = next_account_info(account_info_iter)?;
    let campaign_account = next_account_info(account_info_iter)?;

    let mut campaign_data = AirdropCampaign::try_from_slice(&campaign_account.data.borrow())?;
    if campaign_data.owner != *owner_account.key {
        return Err(AirdropError::InvalidCampaignOwner.into());
    }

    campaign_data.is_active = false;
    campaign_data.end_time = solana_program::clock::Clock::get()?.unix_timestamp;
    campaign_data.serialize(&mut *campaign_account.data.borrow_mut())?;

    Ok(())
}

fn process_claim_airdrop(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let claimer_account = next_account_info(account_info_iter)?;
    let campaign_account = next_account_info(account_info_iter)?;
    let whitelist_account = next_account_info(account_info_iter)?;
    let token_account = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;

    let mut campaign_data = AirdropCampaign::try_from_slice(&campaign_account.data.borrow())?;
    
    if !campaign_data.is_active {
        return Err(AirdropError::CampaignNotActive.into());
    }

    if campaign_data.claimed_count >= campaign_data.max_recipients {
        return Err(AirdropError::MaxRecipientsReached.into());
    }

    if campaign_data.whitelist_required {
        let whitelist_entry = WhitelistEntry::try_from_slice(&whitelist_account.data.borrow())?;
        if whitelist_entry.wallet != *claimer_account.key {
            return Err(AirdropError::NotWhitelisted.into());
        }
        if whitelist_entry.has_claimed {
            return Err(AirdropError::AlreadyClaimed.into());
        }
    }

    // Transfer tokens
    solana_program::program::invoke(
        &token_instruction::transfer(
            token_program.key,
            token_account.key,
            claimer_account.key,
            &campaign_data.owner,
            &[&campaign_data.owner],
            campaign_data.amount_per_recipient,
        )?,
        &[
            token_account.clone(),
            claimer_account.clone(),
            owner_account.clone(),
            token_program.clone(),
        ],
    )?;

    campaign_data.claimed_count += 1;
    campaign_data.serialize(&mut *campaign_account.data.borrow_mut())?;

    if campaign_data.whitelist_required {
        let mut whitelist_entry = WhitelistEntry::try_from_slice(&whitelist_account.data.borrow())?;
        whitelist_entry.has_claimed = true;
        whitelist_entry.serialize(&mut *whitelist_account.data.borrow_mut())?;
    }

    Ok(())
}

fn process_withdraw_remaining_tokens(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let owner_account = next_account_info(account_info_iter)?;
    let campaign_account = next_account_info(account_info_iter)?;
    let token_account = next_account_info(account_info_iter)?;
    let destination_account = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;

    let campaign_data = AirdropCampaign::try_from_slice(&campaign_account.data.borrow())?;
    if campaign_data.owner != *owner_account.key {
        return Err(AirdropError::InvalidCampaignOwner.into());
    }

    if campaign_data.is_active {
        return Err(AirdropError::CampaignNotActive.into());
    }

    // Transfer remaining tokens
    let remaining_amount = campaign_data.total_amount
        .checked_sub(campaign_data.claimed_count
            .checked_mul(campaign_data.amount_per_recipient)
            .unwrap())
        .unwrap();

    solana_program::program::invoke(
        &token_instruction::transfer(
            token_program.key,
            token_account.key,
            destination_account.key,
            &campaign_data.owner,
            &[&campaign_data.owner],
            remaining_amount,
        )?,
        &[
            token_account.clone(),
            destination_account.clone(),
            owner_account.clone(),
            token_program.clone(),
        ],
    )?;

    Ok(())
}
