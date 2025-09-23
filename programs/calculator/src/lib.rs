use anchor_lang::prelude::*;

declare_id!("FKzDRyWenRzZUVX2GUEohkFcYcN7zZBg5xjGYTvPgkp8");

#[program]
pub mod calculator {
    use super::*;
    pub fn init(ctx: Context<Initialize>,init_num:u32) -> Result<()> {
        ctx.accounts.account.count =init_num;
        Ok(())
    }

    pub fn add(ctx:Context<Add>,in_num:u32)->Result<()>{
        ctx.accounts.account.count=ctx.accounts.account.count+in_num;
        Ok(())
    }
    pub fn double(ctx:Context<Double>)->Result<()>{
        ctx.accounts.account.count=ctx.accounts.account.count*2;
        Ok(())
    }

    pub fn sub(ctx: Context<Sub>,num:u32)->Result<()>{
ctx.accounts.account.count = ctx.accounts.account.count-num;

        Ok(())
    }
    pub fn halve(ctx:Context<Halve>)->Result<()>{
        ctx.accounts.account.count=ctx.accounts.account.count*1/2;
        Ok(())
    }

}


#[account]
pub struct DataShape{
    count:u32
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init,payer=signer,space=8+8)]
    pub account:Account<'info,DataShape>,
    #[account(mut)]
   pub signer:Signer<'info>,
   pub system_program:Program<'info,System>
}

#[derive(Accounts)]
pub struct Double<'info>{
    #[account(mut)]
    pub account:Account<'info,DataShape>,
    #[account(mut)]
    pub signer:Signer<'info>
}
#[derive(Accounts)]

pub struct Add<'info>{
    #[account(mut)]
    pub account:Account<'info,DataShape>,
    #[account(mut)]
    pub signer:Signer<'info>
}

#[derive(Accounts)]
pub struct Sub<'info>{
    #[account(mut)]
    pub account:Account<'info,DataShape>,
    #[account(mut)]
    pub signer:Signer<'info>
}
#[derive(Accounts)]
pub struct Halve<'info>{
       #[account(mut)]
    pub account:Account<'info,DataShape>,
    #[account(mut)]
    pub signer:Signer<'info>
}
