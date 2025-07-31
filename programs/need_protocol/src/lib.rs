// programs/need_protocol/src/lib.rs -- ИСПРАВЛЕННАЯ ВЕРСИЯ

use anchor_lang::prelude::*;

declare_id!("E83CPdcp7UkFF7DwYTB3CAjz21gNCADRwLxj3dRA84cj"); // Placeholder Program ID

#[program]
pub mod need_protocol {
    use super::*;

    pub fn create_need(
        ctx: Context<CreateNeed>,
        title: String,
        description: String,
        amount_needed: u64,
        xblago_amount: u64,
    ) -> Result<()> {
        let need = &mut ctx.accounts.need;
        need.creator = *ctx.accounts.creator.key;
        need.title = title;
        need.description = description;
        need.amount_needed = amount_needed;
        need.amount_funded = 0;
        need.xblago_amount = xblago_amount;
        need.status = NeedStatus::Funding as u8;
        // bump теперь присваивается здесь, при инициализации, и уже сохранен в аккаунте
        
        msg!("New Need NFT created: {}", need.title);
        msg!("{} Share tokens are now available for funding.", amount_needed);
        
        Ok(())
    }

    pub fn fund_need(ctx: Context<FundNeed>, amount: u64) -> Result<()> {
        let need = &mut ctx.accounts.need;
        
        require!(need.amount_funded + amount <= need.amount_needed, NeedError::Overfunding);

        // CPIs would go here in a real implementation
        
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
    pub status: u8,
}

#[derive(Accounts)]
#[instruction(title: String, description: String)] // Добавляем инструкции для расчета места
pub struct CreateNeed<'info> {
    #[account(
        init,
        payer = creator,
        space = 8 + 32 + (4 + title.len()) + (4 + description.len()) + 8 + 8 + 8 + 1, // Динамический расчет места
        seeds = [b"need", creator.key().as_ref(), title.as_bytes()], // Используем title для уникальности
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
    /// CHECK: In a real implementation, this would be a specific SPL token account.
    #[account(mut)]
    pub creator_liquid_token_account: AccountInfo<'info>,
    /// CHECK: In a real implementation, this would be a specific SPL token account.
    #[account(mut)]
    pub investor_share_token_account: AccountInfo<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
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