// programs/need_protocol/src/lib.rs

use anchor_lang::prelude::*;

declare_id!("NEEDpL1aToL1aL1aL1aL1aL1aL1aL1aL1aL1aL1aL1a"); // Placeholder Program ID

#[program]
pub mod need_protocol {
    use super::*;

    // Create a new Need, which generates an NFT and prepares for funding
    pub fn create_need(
        ctx: Context<CreateNeed>,
        title: String,
        description: String,
        amount_needed: u64, // In target currency cents (e.g., RUB kopecks)
        xblago_amount: u64, // Pre-calculated amount based on BTC rate
    ) -> Result<()> {
        let need = &mut ctx.accounts.need;
        need.creator = *ctx.accounts.creator.key;
        need.title = title;
        need.description = description;
        need.amount_needed = amount_needed;
        need.amount_funded = 0;
        need.xblago_amount = xblago_amount;
        need.status = NeedStatus::Funding as u8;
        need.bump = *ctx.bumps.get("need").unwrap();

        // Here, you would typically CPI to a token program to mint the NEED_NFT
        // and CPI to a token program to mint the associated "Share" tokens.
        // For simplicity in this text version, we simulate this with logs.
        msg!("New Need NFT created: {}", need.title);
        msg!("{} Share tokens are now available for funding.", amount_needed);
        
        Ok(())
    }

    // Fund a Need. An investor sends a liquid asset (e.g., USDC)
    // and receives "Share" tokens in return.
    pub fn fund_need(ctx: Context<FundNeed>, amount: u64) -> Result<()> {
        let need = &mut ctx.accounts.need;
        
        // Basic check to prevent overfunding
        require!(need.amount_funded + amount <= need.amount_needed, NeedError::Overfunding);

        // 1. Transfer liquid assets (e.g., USDC) from investor to the need's creator
        // This would be a CPI to the SPL token program.
        // transfer_liquid_assets(ctx.accounts.investor, ctx.accounts.creator, amount)?;

        // 2. Mint "Share" tokens to the investor
        // This would be a CPI to the Share token mint.
        // mint_share_tokens(ctx.accounts.investor_share_account, amount)?;
        
        need.amount_funded += amount;
        msg!("Need '{}' funded with {} units.", need.title, amount);

        if need.amount_funded == need.amount_needed {
            need.status = NeedStatus::Funded as u8;
            msg!("Funding for Need '{}' is complete!", need.title);
        }

        Ok(())
    }
}

#[account]
pub struct Need {
    pub creator: Pubkey,
    pub title: String,
    pub description: String,
    pub amount_needed: u64,
    pub amount_funded: u64,
    pub xblago_amount: u64,
    pub status: u8, // 0: Funding, 1: Funded, 2: Canceled
    pub bump: u8,
}

#[derive(Accounts)]
pub struct CreateNeed<'info> {
    #[account(
        init,
        payer = creator,
        space = 8 + 32 + 4 + 100 + 4 + 200 + 8 + 8 + 8 + 1 + 1, // Estimate
        seeds = [b"need", creator.key().as_ref()], // Simplified seed
        bump
    )]
    pub need: Account<'info, Need>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FundNeed<'info> {
    #[account(mut)]
    pub need: Account<'info, Need>,
    #[account(mut)]
    pub investor: Signer<'info>,
    /// CHECK: In a real implementation, this would be the creator's token account.
    #[account(mut)]
    pub creator_liquid_token_account: AccountInfo<'info>,
    /// CHECK: In a real implementation, this would be the investor's share token account.
    #[account(mut)]
    pub investor_share_token_account: AccountInfo<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub enum NeedStatus {
    Funding,
    Funded,
    Canceled,
}

#[error_code]
pub enum NeedError {
    #[msg("This need is already fully funded.")]
    Overfunding,
}