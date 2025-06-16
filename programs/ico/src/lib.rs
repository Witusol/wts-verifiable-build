// lib.rs (salinan dari proyek ICO sebelumnya)
use anchor_spl::token::{Token, TokenAccount, Mint};
use anchor_lang::prelude::*;

declare_id!("GRNddYRgpzmaQMdujW5HfyeFFdqxWLUVpQZxVqk9AgEq");

#[program]
mod ico {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, price_per_token: u64, presale_end_timestamp: i64) -> Result<()> {
        let ico_state = &mut ctx.accounts.ico_state;
        ico_state.authority = *ctx.accounts.authority.key;
        ico_state.token_mint = ctx.accounts.token_mint.key();
        ico_state.token_vault = ctx.accounts.token_vault.key();
        ico_state.price_per_token = price_per_token;
        ico_state.presale_end_timestamp = presale_end_timestamp;
        Ok(())
    }

    pub fn buy_tokens(ctx: Context<BuyTokens>, amount: u64) -> Result<()> {
        // Pembelian token
        Ok(())
    }

    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        // Klaim token
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 128)]
    pub ico_state: Account<'info, IcoState>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub token_mint: Account<'info, Mint>,
    /// CHECK: This is safe because we only store the key
    pub token_vault: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct BuyTokens<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(mut)]
    pub ico_state: Account<'info, IcoState>,
}

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    pub claimer: Signer<'info>,
    #[account(mut)]
    pub ico_state: Account<'info, IcoState>,
}

#[account]
pub struct IcoState {
    pub authority: Pubkey,
    pub token_mint: Pubkey,
    pub token_vault: Pubkey,
    pub price_per_token: u64,
    pub presale_end_timestamp: i64,
}
