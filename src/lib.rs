// programs/good_token/src/lib.rs

use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

declare_id!("GOODpL1aToL1aL1aL1aL1aL1aL1aL1aL1aL1aL1aL1a"); // Placeholder Program ID

#[program]
pub mod good_token {
    use super::*;

    // This function can only be called by the central BLAGO authority (the DAO in the future)
    // It mints a new GOOD token to a user's associated token account.
    // This represents an act of good being recognized.
    pub fn award_good(ctx: Context<AwardGood>, amount: u64) -> Result<()> {
        msg!("Awarding {} GOOD to user {}", amount, ctx.accounts.user_account.key());

        anchor_spl::token::mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.user_account.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info(),
                },
            ),
            amount,
        )?;
        
        msg!("Successfully awarded GOOD.");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct AwardGood<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub user_account: Account<'info, TokenAccount>,
    /// CHECK: This is the central minting authority, validated by the program logic.
    pub mint_authority: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

// Note: The logic to make this token "Soulbound" (non-transferable)
// will be enforced by a separate token-extension program or by the frontend logic,
// which will not provide an interface for transfers.
// The core minting logic is kept simple here.