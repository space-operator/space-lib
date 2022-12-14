use crate::{error::HostError, http, json};

pub struct Solana {
    url: String,
}

impl Solana {
    pub fn new<T>(url: T) -> Self where T: Into<String> {
        Self { url: url.into() }
    }

    pub fn get_account_info(&self, pubkey: &str) -> Result<http::Response, HostError> {
        http::Request::post(&self.url).send_json(json! {
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "getAccountInfo",
                "params": [
                    pubkey,
                ]
            }
        })
    }

    pub fn get_balance(&self, pubkey: &str) -> Result<http::Response, HostError> {
        http::Request::post(&self.url).send_json(json! {
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "getBalance",
                "params": [
                    pubkey,
                ]
            }
        })
    }

    pub fn get_block(&self, slot: u64) -> Result<http::Response, HostError> {
        http::Request::post(&self.url).send_json(json! {
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "getBlock",
                "params": [
                    slot,
                ]
            }
        })
    }

    pub fn get_block_height(&self) -> Result<http::Response, HostError> {
        http::Request::post(&self.url).send_json(json! {
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "getBlockHeight",
            }
        })
    }

    pub fn get_block_production(&self) -> Result<http::Response, HostError> {
        http::Request::post(&self.url).send_json(json! {
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "getBlockProduction",
            }
        })
    }

    pub fn get_block_commitment(&self, slot: u64) -> Result<http::Response, HostError> {
        http::Request::post(&self.url).send_json(json! {
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "getBlockCommitment",
                "params": [
                    slot,
                ]
            }
        })
    }

    pub fn get_blocks(&self, start_slot: u64, end_slot: Option<u64>) -> Result<http::Response, HostError> {
        http::Request::post(&self.url).send_json(json! {
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "getBlocks",
                "params": [
                    start_slot,
                    end_slot,
                ]
            }
        })
    }

    pub fn get_blocks_with_limit(&self, start_slot: u64, limit: u64) -> Result<http::Response, HostError> {
        http::Request::post(&self.url).send_json(json! {
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "getBlocksWithLimit",
                "params": [
                    start_slot,
                    limit,
                ]
            }
        })
    }

    pub fn get_block_time(&self, slot: u64) -> Result<http::Response, HostError> {
        http::Request::post(&self.url).send_json(json! {
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "getBlockTime",
                "params": [
                    slot,
                ]
            }
        })
    }

    pub fn get_cluster_nodes(&self) -> Result<http::Response, HostError> {
        http::Request::post(&self.url).send_json(json! {
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "getClusterNodes",
            }
        })
    }

    pub fn get_epoch_info(&self) -> Result<http::Response, HostError> {
        http::Request::post(&self.url).send_json(json! {
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "getEpochInfo",
            }
        })
    }

    pub fn get_epoch_schedule(&self) -> Result<http::Response, HostError> {
        http::Request::post(&self.url).send_json(json! {
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "getEpochSchedule",
            }
        })
    }

    pub fn get_fee_for_message(&self, message: &str) -> Result<http::Response, HostError> {
        http::Request::post(&self.url).send_json(json! {
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "getFeeForMessage",
                "params": [
                    message,
                ]
            }
        })
    }

    pub fn get_first_available_block(&self) -> Result<http::Response, HostError> {
        http::Request::post(&self.url).send_json(json! {
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "getFirstAvailableBlock",
            }
        })
    }

    pub fn get_genesis_hash(&self) -> Result<http::Response, HostError> {
        http::Request::post(&self.url).send_json(json! {
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "getGenesisHash",
            }
        })
    }

    pub fn get_health(&self) -> Result<http::Response, HostError> {
        http::Request::post(&self.url).send_json(json! {
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "getHealth",
            }
        })
    }

    pub fn get_highest_snapshot_slot(&self) -> Result<http::Response, HostError> {
        http::Request::post(&self.url).send_json(json! {
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "getHighestSnapshotSlot",
            }
        })
    }

    pub fn get_identity(&self) -> Result<http::Response, HostError> {
        http::Request::post(&self.url).send_json(json! {
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "getIdentity",
            }
        })
    }

    pub fn get_inflation_governor(&self) -> Result<http::Response, HostError> {
        http::Request::post(&self.url).send_json(json! {
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "getInflationGovernor",
            }
        })
    }

    pub fn get_inflation_rate(&self) -> Result<http::Response, HostError> {
        http::Request::post(&self.url).send_json(json! {
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "getInflationRate",
            }
        })
    }

    pub fn get_inflation_reward(&self, pubkeys: Vec<&str>) -> Result<http::Response, HostError> {
        http::Request::post(&self.url).send_json(json! {
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "getInflationReward",
                "params": [
                    pubkeys,
                ]
            }
        })
    }

    pub fn get_largest_accounts(&self) -> Result<http::Response, HostError> {
        http::Request::post(&self.url).send_json(json! {
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "getLargestAccounts",
            }
        })
    }

    pub fn get_latest_blockhash(&self) -> Result<http::Response, HostError> {
        http::Request::post(&self.url).send_json(json! {
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "getLatestBlockhash",
            }
        })
    }

    pub fn get_leader_schedule(&self) -> Result<http::Response, HostError> {
        http::Request::post(&self.url).send_json(json! {
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "getLeaderSchedule",
            }
        })
    }

    pub fn get_max_retransmit_slot(&self) -> Result<http::Response, HostError> {
        http::Request::post(&self.url).send_json(json! {
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "getMaxRetransmitSlot",
            }
        })
    }

    pub fn get_max_shred_insert_slot(&self) -> Result<http::Response, HostError> {
        http::Request::post(&self.url).send_json(json! {
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "getMaxShredInsertSlot",
            }
        })
    }

    pub fn get_minimum_balance_for_rent_exemption(&self) -> Result<http::Response, HostError> {
        http::Request::post(&self.url).send_json(json! {
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "getMinimumBalanceForRentExemption",
            }
        })
    }

    pub fn get_multiple_accounts(&self, pubkeys: Vec<&str>) -> Result<http::Response, HostError> {
        http::Request::post(&self.url).send_json(json! {
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "getMultipleAccounts",
                "params": [
                    pubkeys,
                ]
            }
        })
    }

    pub fn get_program_accounts(&self, pubkey: &str) -> Result<http::Response, HostError> {
        http::Request::post(&self.url).send_json(json! {
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "getProgramAccounts",
                "params": [
                    pubkey,
                ]
            }
        })
    }

    pub fn get_recent_performance_samples(&self) -> Result<http::Response, HostError> {
        http::Request::post(&self.url).send_json(json! {
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "getRecentPerformanceSamples",
            }
        })
    }

    pub fn get_signatures_for_address(&self, pubkey: &str) -> Result<http::Response, HostError> {
        http::Request::post(&self.url).send_json(json! {
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "getSignaturesForAddress",
                "params": [
                    pubkey,
                ]
            }
        })
    }

    pub fn get_signature_statuses(&self, signatures: Vec<&str>) -> Result<http::Response, HostError> {
        http::Request::post(&self.url).send_json(json! {
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "getSignatureStatuses",
                "params": [
                    signatures,
                ]
            }
        })
    }

    pub fn get_slot(&self) -> Result<http::Response, HostError> {
        http::Request::post(&self.url).send_json(json! {
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "getSlot",
            }
        })
    }

    pub fn get_slot_leader(&self) -> Result<http::Response, HostError> {
        http::Request::post(&self.url).send_json(json! {
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "getSlotLeader",
            }
        })
    }

    pub fn get_slot_leaders(&self, start_slot: u64, limit: u64) -> Result<http::Response, HostError> {
        http::Request::post(&self.url).send_json(json! {
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "getSlotLeaders",
                "params": [
                    start_slot,
                    limit,
                ]
            }
        })
    }

    pub fn get_stake_activation(&self, pubkey: &str) -> Result<http::Response, HostError> {
        http::Request::post(&self.url).send_json(json! {
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "getStakeActivation",
                "params": [
                    pubkey,
                ]
            }
        })
    }

    pub fn get_stake_minimum_delegation(&self) -> Result<http::Response, HostError> {
        http::Request::post(&self.url).send_json(json! {
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "getStakeMinimumDelegation",
            }
        })
    }

    pub fn get_supply(&self) -> Result<http::Response, HostError> {
        http::Request::post(&self.url).send_json(json! {
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "getSupply",
            }
        })
    }

    pub fn get_token_account_balance(&self, pubkey: &str) -> Result<http::Response, HostError> {
        http::Request::post(&self.url).send_json(json! {
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "getTokenAccountBalance",
                "params": [
                    pubkey,
                ]
            }
        })
    }

    pub fn get_token_accounts_by_delegate_mint(&self, pubkey: &str, mint: &str) -> Result<http::Response, HostError> {
        http::Request::post(&self.url).send_json(json! {
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "getTokenAccountsByDelegate",
                "params": [
                    pubkey,
                    {
                        "mint": mint
                    },
                ]
            }
        })
    }

    pub fn get_token_accounts_by_delegate_program(
        &self,
        pubkey: &str,
        program: &str,
    ) -> Result<http::Response, HostError> {
        http::Request::post(&self.url).send_json(json! {
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "getTokenAccountsByDelegate",
                "params": [
                    pubkey,
                    {
                        "programId": program
                    },
                ]
            }
        })
    }

    pub fn get_token_accounts_by_owner_mint(&self, pubkey: &str, mint: &str) -> Result<http::Response, HostError> {
        http::Request::post(&self.url).send_json(json! {
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "getTokenAccountsByOwner",
                "params": [
                    pubkey,
                    {
                        "mint": mint
                    },
                ]
            }
        })
    }

    pub fn get_token_accounts_by_owner_program(
        &self,
        pubkey: &str,
        program: &str,
    ) -> Result<http::Response, HostError> {
        http::Request::post(&self.url).send_json(json! {
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "getTokenAccountsByOwner",
                "params": [
                    pubkey,
                    {
                        "programId": program
                    },
                ]
            }
        })
    }

    pub fn get_token_largest_accounts(&self, pubkey: &str) -> Result<http::Response, HostError> {
        http::Request::post(&self.url).send_json(json! {
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "getTokenLargestAccounts",
                "params": [
                    pubkey,
                ]
            }
        })
    }

    pub fn get_token_supply(&self, pubkey: &str) -> Result<http::Response, HostError> {
        http::Request::post(&self.url).send_json(json! {
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "getTokenSupply",
                "params": [
                    pubkey,
                ]
            }
        })
    }

    pub fn get_transaction(&self, signature: &str) -> Result<http::Response, HostError> {
        http::Request::post(&self.url).send_json(json! {
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "getTransaction",
                "params": [
                    signature,
                ]
            }
        })
    }

    pub fn get_transaction_count(&self) -> Result<http::Response, HostError> {
        http::Request::post(&self.url).send_json(json! {
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "getTransactionCount",
            }
        })
    }

    pub fn get_vote_accounts(&self) -> Result<http::Response, HostError> {
        http::Request::post(&self.url).send_json(json! {
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "getVoteAccounts",
            }
        })
    }

    pub fn is_blockhash_valid(&self, blockhash: &str) -> Result<http::Response, HostError> {
        http::Request::post(&self.url).send_json(json! {
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "isBlockhashValid",
                "params": [
                    blockhash,
                ]
            }
        })
    }

    pub fn minimum_ledger_slot(&self) -> Result<http::Response, HostError> {
        http::Request::post(&self.url).send_json(json! {
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "minimumLedgerSlot",
            }
        })
    }

    pub fn request_airdrop(&self, pubkey: &str, lamports: u64) -> Result<http::Response, HostError> {
        http::Request::post(&self.url).send_json(json! {
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "requestAirdrop",
                "params": [
                    pubkey,
                    lamports,
                ]
            }
        })
    }

    pub fn send_transaction(&self, transaction: &str) -> Result<http::Response, HostError> {
        http::Request::post(&self.url).send_json(json! {
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "sendTransaction",
                "params": [
                    transaction,
                ]
            }
        })
    }

    pub fn simulate_transaction(&self, transaction: &str) -> Result<http::Response, HostError> {
        http::Request::post(&self.url).send_json(json! {
            {
                "jsonrpc": "2.0",
                "id": 1,
                "method": "simulateTransaction",
                "params": [
                    transaction,
                ]
            }
        })
    }
}
