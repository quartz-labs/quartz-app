use anchor_lang::prelude::*;
use solana_program::pubkey;

pub const ANCHOR_DISCRIMINATOR: usize = 8;
pub const PUBKEY_SIZE: usize = 32;
pub const U8_SIZE: usize = 1;

pub const DRIFT_MARKET_INDEX_USDC: u16 = 0;
pub const DRIFT_MARKET_INDEX_SOL: u16 = 1;


// // Local
// #[constant]
// pub const USDC_MINT_ADDRESS: Pubkey = pubkey!("envrJbV6GbhBTi8Pu6h9MwNViLuAmu3mFFRq7gE9Cp3");

// Devnet
#[constant]
// pub const USDC_MINT_ADDRESS: Pubkey = pubkey!("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU");
pub const USDC_MINT_ADDRESS: Pubkey = pubkey!("8zGuJQqwhZafTah7Uc7Z4tXRnguqkn5KLFAP8oV6PHe2");

// // Mainnet
// #[constant]
// pub const USDC_MINT_ADDRESS: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");

// // Local
// #[constant]
// pub const WSOL_MINT_ADDRESS: Pubkey = pubkey!("");

// Devnet + Mainnet
#[constant]
pub const WSOL_MINT_ADDRESS: Pubkey = pubkey!("So11111111111111111111111111111111111111112");
