// programs/good_token/src/lib.rs -- ИСПРАВЛЕННАЯ ВЕРСИЯ

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, MintTo, Burn};

declare_id!("ChxnGdaewCaV3iyWkhtmSxFEYhD6x1L3yYVLZKRaqDLD"); // Placeholder Program ID

#[program]
pub mod good_token {
    use super::*;

    // Initialize the central mint for GOOD tokens. Can only be called once.
    pub fn initialize_mint(ctx: Context<InitializeMint>) -> Result<()> {
        msg!("GOOD token mint initialized.");
        Ok(())
    }

    // Award GOOD tokens to a user. Can only be called by the DAO/authority.
    pub fn award_good(ctx: Context<AwardGood>, amount: u64) -> Result<()> {
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.mint_authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        token::mint_to(cpi_ctx, amount)?;
        
        msg!("Awarded {} GOOD to user {}", amount, ctx.accounts.user_token_account.owner);
        Ok(())
    }

    // Burn GOOD tokens when a user stakes them for a vote.
    // In our model, we might just lock them, but burning is another possibility.
    // For now, let's include it as a utility function.
    pub fn burn_good(ctx: Context<BurnGood>, amount: u64) -> Result<()> {
        let cpi_accounts = Burn {
            mint: ctx.accounts.mint.to_account_info(),
            from: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.user_authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        token::burn(cpi_ctx, amount)?;

        msg!("Burned {} GOOD from user {}", amount, ctx.accounts.user_authority.key());
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeMint<'info> {
    #[account(
        init,
        payer = payer,
        mint::decimals = 0, // GOOD are whole units, no decimals
        mint::authority = mint_authority,
    )]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: The authority can be a PDA or a specific keypair.
    pub mint_authority: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct AwardGood<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    /// CHECK: Validated as the mint_authority for the `mint` account.
    pub mint_authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct BurnGood<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(
        mut,
        constraint = user_token_account.owner == *user_authority.key
    )]
    pub user_token_account: Account<'info, TokenAccount>,
    pub user_authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}