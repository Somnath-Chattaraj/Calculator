use anchor_lang::prelude::*;

declare_id!("69fPaGPVks2mVUzPBppPAUXYmrMNGZJMJif1oQfftK6r");

#[program]
pub mod mycalculator_dapp {
    use super::*;
    
    pub fn create(ctx: Context<Create>, init_message: String) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        let msg_bytes = init_message.as_bytes();
        let length = msg_bytes.len().min(32);
        calculator.greeting[..length].copy_from_slice(&msg_bytes[..length]);
        Ok(())
    }

    pub fn add(ctx: Context<Addition>, num1: i64, num2: i64) -> Result<()> {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 + num2;
        calculator.remainder = 0; // Initialize or update this as needed
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = user, space = 8 + 40 + 16)] // 8 bytes for discriminator, 40 bytes for string, 16 bytes for two i64 fields
    pub calculator: Account<'info, Calculator>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Addition<'info> {
    #[account(mut)]
    pub calculator: Account<'info, Calculator>,
}

#[account]
pub struct Calculator {
    pub greeting: [u8; 32], // Fixed-size array for storing the greeting (32 bytes)
    pub result: i64,
    pub remainder: i64,
}
