use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint, Transfer};

pub mod account;
pub mod constants;
pub mod error;

use account::*;

declare_id!("AgYi8P676GfXo1d5Wik4JTbcu5FBQG5ULPoQpXUKNtmG");

#[program]
pub mod firebird {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let global_authority = &mut ctx.accounts.global_authority;
        global_authority.admin = ctx.accounts.admin.key();
        Ok(())
    }
    
    pub fn user_token_pool_initialize(ctx: Context<UserStateInitialize>) -> Result<()> {
        let user_state = &mut ctx.accounts.user_state;
        user_state.amount = 0;
        user_state.mint = ctx.accounts.token_mint.key();
        Ok(())
    }

    pub fn deposit(ctx: Context<DepositToken>, amount: u64) -> Result<()> {
        let user_state = &mut ctx.accounts.user_state;
        let token_pool_info = &mut &ctx.accounts.token_pool;
        let user_token_account_info = &mut &ctx.accounts.user_token_account;
        let token_program = &mut &ctx.accounts.token_program;

        let cpi_accounts = Transfer {
            from: user_token_account_info.to_account_info().clone(),
            to: token_pool_info.to_account_info().clone(),
            authority: ctx.accounts.signer.to_account_info().clone(),
        };
        token::transfer(
            CpiContext::new(token_program.clone().to_account_info(), cpi_accounts), 
            amount
        )?;

        user_state.amount += amount;
        Ok(())
    }

    pub fn withdraw(ctx: Context<WithdrawToken>, amount: u64) -> Result<()> {
        let user_state = &mut ctx.accounts.user_state;
        let token_program = &mut &ctx.accounts.token_program;

        let seeds = &[
            "Global Pool".as_bytes(),
            &[ctx.bumps.global_authority]
        ];

        let cpi_accounts = Transfer {
            from: ctx.accounts.token_pool.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.global_authority.to_account_info(),
        };
        token::transfer(
            CpiContext::new_with_signer(token_program.clone().to_account_info(), cpi_accounts, &[&seeds[..]]), 
            amount
        )?;

        user_state.amount -= amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds = [b"Global Pool"],
        bump,
        payer = admin,
        space = 8 + 32
    )]
    pub global_authority: Account<'info, GlobalPool>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct UserStateInitialize<'info> {
    #[account(
        init,
        seeds = [token_mint.key().as_ref(), signer.key().as_ref()],
        bump,
        payer = signer,
        space = 8 + 8 + 32
    )]
    pub user_state: Account<'info, UserState>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub token_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct DepositToken<'info> {
    #[account(
        mut,
        seeds = [token_mint.key().as_ref(), signer.key().as_ref()],
        bump,
    )]
    pub user_state: Account<'info, UserState>,

    #[account(
        mut,
        constraint = token_pool.mint == token_mint.key(),
        constraint = token_pool.owner == global_authority.key()
    )]
    pub token_pool: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = user_token_account.mint == token_mint.key(),
        constraint = user_token_account.owner == signer.key(),
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        seeds = [b"Global Pool"],
        bump,
    )]
    pub global_authority: Account<'info, GlobalPool>,
    
    pub token_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>
}

#[derive(Accounts)]
pub struct WithdrawToken<'info> {
    #[account(
        mut,
        seeds = [token_mint.key().as_ref(), signer.key().as_ref()],
        bump,
    )]
    pub user_state: Account<'info, UserState>,

    #[account(
        mut,
        constraint = token_pool.mint == token_mint.key(),
        constraint = token_pool.owner == global_authority.key()
    )]
    pub token_pool: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = user_token_account.mint == token_mint.key(),
        constraint = user_token_account.owner == signer.key(),
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        seeds = [b"Global Pool"],
        bump,
    )]
    pub global_authority: Account<'info, GlobalPool>,
    
    pub token_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>
}
