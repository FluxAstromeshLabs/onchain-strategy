use cosmwasm_std::{from_json, Binary, StdError, Uint64};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub mod raydium {
    use crate::astromesh::{FISInput, FISInstruction, Pool, Swap};
    use cosmwasm_schema::cw_serde;
    use cosmwasm_std::{to_json_vec, Binary, Int256, StdError};
    use tiny_keccak::{Hasher, Keccak};

    use super::{Account, Instruction, InstructionAccount, MsgTransaction, Pubkey, TokenAccount};

    pub const RAYDIUM: &str = "raydium";
    pub const SPL_TOKEN_2022: &str = "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb";
    pub const CPMM_PROGRAM_ID: &str = "CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C";
    pub const ASSOCIATED_TOKEN_PROGRAM_ID: &str = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL";
    pub const BPS: i128 = 1000000i128;

    #[derive(Clone)]
    pub struct PoolAccounts {
        pub authority_account: String,
        pub amm_config_account: String,
        pub pool_state_account: String,
        pub token0_mint: String,
        pub token1_mint: String,
        pub token0_vault: String,
        pub token1_vault: String, // can be calculated
        pub observer_state: String,
    }

    pub fn get_pool_accounts_by_name(pool_name: &String) -> Result<PoolAccounts, StdError> {
        match pool_name.as_str() {
            "btc-usdt" => Ok(PoolAccounts {
                authority_account: "GpMZbSM2GgvTKHJirzeGfMFoaZ8UR2X7F4v8vHTvxFbL".to_string(),
                amm_config_account: "D4FPEruKEHrG5TenZ2mpDGEfu1iUvTiqBxvpU8HLBvC2".to_string(),
                pool_state_account: "5qUshuBSTpuMu5c1C1Fxq8uJ7Emhn9AAtQwVJfEXAPmy".to_string(),
                token0_mint: "9kWnPUAkspGW6qGPPah1aAdH316nkiJhow5neRs5YDej".to_string(),
                token1_mint: "HwkqUQaXocRwNLGX2qKmC3Sk4uTVxmzmCEAEHDwSj4KQ".to_string(),
                token0_vault: "9U5Lpfmc6u1rCRAfzGe883KnK5Avm76zX4te6sexvCEk".to_string(),
                token1_vault: "UURmKznoUTh8Dt9wgyusq6u1ETuY8Zj79NFAtfQJ7HB".to_string(),
                observer_state: "FXqXrt2xDrxg7J5wdXrTbB2hCGajSzXLvwvc4x3Uw7i".to_string(),
            }),
            name => Err(StdError::not_found(name)),
        }
    }

    pub fn get_denom(denom: &String) -> String {
        match denom.as_str() {
            "btc" => "9kWnPUAkspGW6qGPPah1aAdH316nkiJhow5neRs5YDej".to_string(),
            "usdt" => "HwkqUQaXocRwNLGX2qKmC3Sk4uTVxmzmCEAEHDwSj4KQ".to_string(),
            _ => denom.clone(),
        }
    }

    pub fn swap_base_input(
        sender: String,
        amount_in: u64,
        min_amount_out: u64,
        sender_svm_account: String,
        authority_account: String,
        amm_config_account: String,
        pool_state_account: String,
        input_token_account: String,
        output_token_account: String,
        input_vault: String,
        output_vault: String,
        input_token_mint: String,
        output_token_mint: String,
        observer_state: String,
    ) -> MsgTransaction {
        let accounts = vec![
            sender_svm_account,
            authority_account,
            amm_config_account,
            pool_state_account,
            input_token_account,
            output_token_account,
            input_vault,
            output_vault,
            SPL_TOKEN_2022.to_string(),
            input_token_mint,
            output_token_mint,
            observer_state,
            CPMM_PROGRAM_ID.to_string(),
        ];

        let mut data_bz: Vec<u8> = vec![143, 190, 90, 218, 196, 30, 51, 222];
        data_bz.extend(amount_in.to_le_bytes());
        data_bz.extend(min_amount_out.to_le_bytes());

        MsgTransaction {
            // ty: "flux.svm.v1beta1.MsgTransaction".to_string(),
            sender,
            accounts,
            instructions: vec![Instruction {
                program_index: vec![12],
                accounts: vec![
                    InstructionAccount {
                        id_index: 0,
                        caller_index: 0,
                        callee_index: 0,
                        is_signer: true,
                        is_writable: true,
                    },
                    InstructionAccount {
                        id_index: 1,
                        caller_index: 1,
                        callee_index: 1,
                        is_signer: false,
                        is_writable: false,
                    },
                    InstructionAccount {
                        id_index: 2,
                        caller_index: 2,
                        callee_index: 2,
                        is_signer: false,
                        is_writable: false,
                    },
                    InstructionAccount {
                        id_index: 3,
                        caller_index: 3,
                        callee_index: 3,
                        is_signer: false,
                        is_writable: true,
                    },
                    InstructionAccount {
                        id_index: 4,
                        caller_index: 4,
                        callee_index: 4,
                        is_signer: false,
                        is_writable: true,
                    },
                    InstructionAccount {
                        id_index: 5,
                        caller_index: 5,
                        callee_index: 5,
                        is_signer: false,
                        is_writable: true,
                    },
                    InstructionAccount {
                        id_index: 6,
                        caller_index: 6,
                        callee_index: 6,
                        is_signer: false,
                        is_writable: true,
                    },
                    InstructionAccount {
                        id_index: 7,
                        caller_index: 7,
                        callee_index: 7,
                        is_signer: false,
                        is_writable: true,
                    },
                    InstructionAccount {
                        id_index: 8,
                        caller_index: 8,
                        callee_index: 8,
                        is_signer: false,
                        is_writable: false,
                    },
                    InstructionAccount {
                        id_index: 8,
                        caller_index: 8,
                        callee_index: 8,
                        is_signer: false,
                        is_writable: false,
                    },
                    InstructionAccount {
                        id_index: 9,
                        caller_index: 9,
                        callee_index: 10,
                        is_signer: false,
                        is_writable: false,
                    },
                    InstructionAccount {
                        id_index: 10,
                        caller_index: 10,
                        callee_index: 11,
                        is_signer: false,
                        is_writable: false,
                    },
                    InstructionAccount {
                        id_index: 11,
                        caller_index: 11,
                        callee_index: 12,
                        is_signer: false,
                        is_writable: true,
                    },
                ],
                data: Binary::from(data_bz),
            }],
            compute_budget: 10_000_000,
        }
    }

    fn keccak256(input: &[u8]) -> Vec<u8> {
        let mut hash = Keccak::v256();
        hash.update(input);
        let mut output = Vec::with_capacity(32);
        hash.finalize(output.as_mut_slice());
        output
    }

    #[cw_serde]
    #[derive(Default)]
    pub struct RaydiumPool {
        pub dex_name: String,
        pub denom_plane: String,
        pub a: Int256,
        pub b: Int256,
        pub fee_rate: Int256,
        pub denom_a: String,
        pub denom_b: String,
    }

    impl RaydiumPool {
        pub fn from_fis(input: &FISInput) -> Result<Self, StdError> {
            let token_0_vault_account = Account::from_json_bytes(
                input
                    .data
                    .get(0)
                    .ok_or(StdError::not_found("expected account 0"))?,
            )?;
            let token_1_vault_account = Account::from_json_bytes(
                input
                    .data
                    .get(1)
                    .ok_or(StdError::not_found("expected account 1"))?,
            )?;
            let token_0_info = TokenAccount::unpack(token_0_vault_account.data.as_slice())?;
            let token_1_info = TokenAccount::unpack(token_1_vault_account.data.as_slice())?;
            // TODO: more constraint as validate basic
            let (mut a, mut b) = (token_0_info.amount, token_1_info.amount);
            // we always swap from usdt so let it be the first
            if token_0_info.mint.to_string() != get_denom(&"usdt".to_string()) {
                (a, b) = (b, a)
            }

            Ok(Self {
                dex_name: RAYDIUM.to_string(),
                denom_plane: "SVM".to_string(),
                a: Int256::from_i128(a as i128),
                b: Int256::from_i128(b as i128),
                fee_rate: Int256::from(1000i128),
                denom_a: token_0_info.mint.to_string(),
                denom_b: token_1_info.mint.to_string(),
            })
        }
    }

    impl Pool for RaydiumPool {
        fn dex_name(&self) -> String {
            self.dex_name.clone()
        }

        fn denom_plane(&self) -> String {
            self.denom_plane.clone()
        }

        fn a(&self) -> Int256 {
            self.a.clone()
        }

        fn b(&self) -> Int256 {
            self.b.clone()
        }

        fn swap_output(&self, x: Int256, a_for_b: bool) -> (String, Int256) {
            let bps = Int256::from_i128(BPS);
            let x = x * (bps - self.fee_rate) / bps;

            if a_for_b {
                (self.denom_b.clone(), (self.b * x) / (self.a + x))
            } else {
                (self.denom_a.clone(), (self.a * x) / (self.b + x))
            }
        }

        fn compose_swap_fis(&self, swap: &Swap) -> Result<FISInstruction, StdError> {
            // let accounts = swap.raydium_accounts.unwrap();
            let accounts = get_pool_accounts_by_name(&swap.pool_name)?;
            let (_, sender_bz) = bech32::decode(swap.sender.as_str()).unwrap();
            let sender_svm_account: Pubkey =
                Pubkey::from_slice(keccak256(&sender_bz.as_slice()).as_slice())?;
            let input_denom = get_denom(&swap.denom);
            let (mut input_vault, mut output_vault) =
                (accounts.token0_vault, accounts.token1_vault);
            if &input_denom == &accounts.token1_mint {
                (input_vault, output_vault) = (output_vault, input_vault);
            }

            let output_denom = if input_denom == accounts.token0_mint {
                accounts.token1_mint
            } else {
                accounts.token0_mint
            };

            let input_denom_pk = Pubkey::from_string(&input_denom)?;
            let output_denom_pk = Pubkey::from_string(&output_denom)?;
            let token_program = Pubkey::from_string(&SPL_TOKEN_2022.to_string())?;
            let ata_program = Pubkey::from_string(&ASSOCIATED_TOKEN_PROGRAM_ID.to_string())?;

            let (input_token_account, _) = Pubkey::find_program_address(
                &[&sender_svm_account.0, &token_program.0, &input_denom_pk.0],
                &ata_program,
            )
            .unwrap();
            let (output_token_account, _) = Pubkey::find_program_address(
                &[&sender_svm_account.0, &token_program.0, &output_denom_pk.0],
                &ata_program,
            )
            .unwrap();

            let msg = swap_base_input(
                swap.sender.clone(),
                swap.amount.i128() as u64,
                0,
                sender_svm_account.to_string(),
                // sender_svm_account.to_string(),
                accounts.authority_account,
                accounts.amm_config_account,
                accounts.pool_state_account,
                input_token_account.to_string(),
                output_token_account.to_string(),
                input_vault,
                output_vault,
                input_denom,
                output_denom,
                accounts.observer_state,
            );
            Ok(FISInstruction {
                plane: "SVM".to_string(),
                action: "VM_INVOKE".to_string(),
                address: "".to_string(),
                msg: to_json_vec(&msg)?,
            })
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MsgTransaction {
    /// Sender is the address of the actor that signed the message
    pub sender: String,
    /// Accounts are the cosmos addresses that sign this message
    pub accounts: Vec<String>,
    /// Instructions are the instructions for the transaction
    pub instructions: Vec<Instruction>,
    /// ComputeBudget is the budget for computation
    pub compute_budget: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Instruction {
    /// ProgramIndex is a list of program indices
    pub program_index: Vec<u32>,
    /// Accounts are the accounts involved in the instruction
    pub accounts: Vec<InstructionAccount>,
    /// Data is the data for the instruction
    pub data: Binary,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    pub pubkey: Binary,
    pub owner: Binary,
    pub lamports: Uint64, // JSON cdc returns string (with quotes), standard u64 can't be parsed
    pub data: Binary,
    pub executable: bool,
    pub rent_epoch: Uint64,
}

impl Account {
    pub fn from_json_bytes(bz: &[u8]) -> Result<Self, StdError> {
        from_json(bz)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstructionAccount {
    pub id_index: u32,
    pub caller_index: u32,
    pub callee_index: u32,
    pub is_signer: bool,
    pub is_writable: bool,
}

const PDA_MARKER: &[u8; 21] = b"ProgramDerivedAddress";

pub struct Hash(pub(crate) [u8; 32]);

#[derive(Clone, Default)]
pub struct Hasher {
    hasher: Sha256,
}

impl Hasher {
    pub fn hash(&mut self, val: &[u8]) {
        self.hasher.update(val);
    }
    pub fn hashv(&mut self, vals: &[&[u8]]) {
        for val in vals {
            self.hash(val);
        }
    }
    pub fn result(self) -> Hash {
        Hash(self.hasher.finalize().into())
    }
}

pub fn bytes_are_curve_point<T: AsRef<[u8]>>(_bytes: T) -> bool {
    curve25519_dalek::edwards::CompressedEdwardsY::from_slice(_bytes.as_ref())
        .unwrap()
        .decompress()
        .is_some()
}

#[derive(Debug)]
pub struct Pubkey(pub [u8; 32]);

pub enum PubkeyError {
    MaxSeedLengthExceeded,
    InvalidSeeds,
    IllegalOwner,
}

impl Pubkey {
    pub fn to_string(&self) -> String {
        bs58::encode(self.0).into_string()
    }

    pub fn from_slice(bz: &[u8]) -> Result<Self, StdError> {
        if bz.len() != 32 {
            return Err(StdError::generic_err("pubkey must be 32 bytes"));
        }

        let mut pubkey: [u8; 32] = [0; 32];
        pubkey.copy_from_slice(bz);
        Ok(Self(pubkey))
    }

    pub fn from_string(s: &String) -> Result<Self, StdError> {
        let bz = bs58::decode(s.as_str())
            .into_vec()
            .or_else(|e| Err(StdError::generic_err(e.to_string())))?;
        Pubkey::from_slice(bz.as_slice())
    }

    pub fn find_program_address(seeds: &[&[u8]], program_id: &Pubkey) -> Option<(Pubkey, u8)> {
        let mut bump_seed = [u8::MAX];
        for _ in 0..u8::MAX {
            {
                let mut seeds_with_bump = seeds.to_vec();
                seeds_with_bump.push(&bump_seed);
                match Self::create_program_address(&seeds_with_bump, program_id) {
                    Ok(address) => return Some((address, bump_seed[0])),
                    Err(PubkeyError::InvalidSeeds) => (),
                    _ => break,
                }
            }
            bump_seed[0] -= 1;
        }
        None
    }

    pub fn create_program_address(
        seeds: &[&[u8]],
        program_id: &Pubkey,
    ) -> Result<Pubkey, PubkeyError> {
        if seeds.len() > 255 {
            return Err(PubkeyError::MaxSeedLengthExceeded);
        }

        for seed in seeds.iter() {
            if seed.len() > 32 {
                return Err(PubkeyError::MaxSeedLengthExceeded);
            }
        }

        let mut hasher = Hasher::default();
        for seed in seeds.iter() {
            hasher.hash(seed);
        }
        hasher.hashv(&[program_id.0.as_slice(), PDA_MARKER]);
        let hash = hasher.result();

        if bytes_are_curve_point(hash.0) {
            return Err(PubkeyError::InvalidSeeds);
        }

        Ok(Pubkey::from_slice(hash.0.as_slice()).unwrap())
    }
}

// Simplified version of token account
#[derive(Debug)]
pub struct TokenAccount {
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub amount: u64,
}

// "getrandom" dep

impl TokenAccount {
    pub fn unpack(bz: &[u8]) -> Result<TokenAccount, StdError> {
        if bz.len() < 72 {
            return Err(StdError::generic_err("token account size must >= 72 bytes"));
        }

        Ok(TokenAccount {
            mint: Pubkey::from_slice(&bz[0..32])?,
            owner: Pubkey::from_slice(&bz[32..64])?,
            amount: u64::from_le_bytes(bz[64..72].try_into().unwrap()), // we know for sure it's 8 bytes => unwrap() is safe
        })
    }
}