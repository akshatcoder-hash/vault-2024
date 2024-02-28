use anchor_lang::prelude::*;

declare_id!("73kVpBT3u7awKf4N7aW3QqUbbDgb6PKvHN7SgD7YwSXf");

#[program]
pub mod vault_2024 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
