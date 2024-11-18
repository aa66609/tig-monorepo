use crate::{config::ProtocolConfig, serializable_struct_with_getters};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::{HashMap, HashSet};
use tig_utils::{jsonify, u64s_from_str, u8s_from_str};
pub use tig_utils::{Frontier, MerkleBranch, MerkleHash, Point, PreciseNumber, Transaction, U256};

serializable_struct_with_getters! {
    Algorithm {
        id: String,
        details: AlgorithmDetails,
        state: AlgorithmState,
        block_data: Option<AlgorithmBlockData>,
        round_earnings: PreciseNumber,
    }
}
serializable_struct_with_getters! {
    Benchmark {
        id: String,
        details: BenchmarkDetails,
        state: BenchmarkState,
        solution_nonces: Option<HashSet<u64>>,
    }
}
serializable_struct_with_getters! {
    Binary {
        algorithm_id: String,
        details: BinaryDetails,
        state: BinaryState,
    }
}
serializable_struct_with_getters! {
    Block {
        id: String,
        details: BlockDetails,
        data: Option<BlockData>,
        config: Option<ProtocolConfig>,
    }
}
serializable_struct_with_getters! {
    Breakthrough {
        id: String,
        details: BreakthroughDetails,
        state: BreakthroughState,
        block_data: Option<BreakthroughBlockData>,
    }
}
serializable_struct_with_getters! {
    Challenge {
        id: String,
        details: ChallengeDetails,
        state: ChallengeState,
        block_data: Option<ChallengeBlockData>,
    }
}
serializable_struct_with_getters! {
    Delegate {
        id: String,
        details: DelegateDetails,
        state: DelegateState,
    }
}
serializable_struct_with_getters! {
    Deposit {
        id: String,
        details: DepositDetails,
        state: DepositState,
    }
}
serializable_struct_with_getters! {
    Fraud {
        benchmark_id: String,
        state: FraudState,
        allegation: Option<String>,
    }
}
serializable_struct_with_getters! {
    OPoW {
        player_id: String,
        block_data: Option<OPoWBlockData>,
        round_earnings: PreciseNumber,
    }
}
serializable_struct_with_getters! {
    Player {
        id: String,
        details: PlayerDetails,
        state: PlayerState,
        block_data: Option<PlayerBlockData>,
        round_earnings_by_type: HashMap<RewardType, PreciseNumber>,
    }
}
serializable_struct_with_getters! {
    Precommit {
        benchmark_id: String,
        details: PrecommitDetails,
        settings: BenchmarkSettings,
        state: PrecommitState,
    }
}
serializable_struct_with_getters! {
    MerkleProof {
        leaf: OutputData,
        branch: MerkleBranch,
    }
}
serializable_struct_with_getters! {
    Proof {
        benchmark_id: String,
        details: ProofDetails,
        state: ProofState,
        merkle_proofs: Option<Vec<MerkleProof>>,
    }
}
serializable_struct_with_getters! {
    RewardShare {
        id: String,
        details: RewardShareDetails,
        state: RewardShareState,
    }
}
serializable_struct_with_getters! {
    TopUp {
        id: String,
        details: TopUpDetails,
        state: TopUpState,
    }
}
serializable_struct_with_getters! {
    Vote {
        id: String,
        details: VoteDetails,
        state: VoteState,
    }
}

// Algorithm child structs
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AlgorithmType {
    Wasm,
    Ptx,
}
serializable_struct_with_getters! {
    AlgorithmDetails {
        name: String,
        player_id: String,
        challenge_id: String,
        breakthrough_id: Option<String>,
        r#type: AlgorithmType,
        fee_paid: PreciseNumber,
    }
}
serializable_struct_with_getters! {
    AlgorithmState {
        block_confirmed: u32,
        round_submitted: u32,
        round_pushed: Option<u32>,
        round_active: Option<u32>,
        round_merged: Option<u32>,
        banned: bool,
    }
}
serializable_struct_with_getters! {
    AlgorithmBlockData {
        num_qualifiers_by_player: HashMap<String, u32>,
        adoption: PreciseNumber,
        merge_points: u32,
        reward: PreciseNumber,
    }
}

// Benchmark child structs
serializable_struct_with_getters! {
    BenchmarkSettings {
        player_id: String,
        block_id: String,
        challenge_id: String,
        algorithm_id: String,
        difficulty: Vec<i32>,
    }
}
impl BenchmarkSettings {
    pub fn calc_seed(&self, rand_hash: &String, nonce: u64) -> [u8; 32] {
        u8s_from_str(&format!("{}_{}_{}", jsonify(&self), rand_hash, nonce))
    }
}
serializable_struct_with_getters! {
    BenchmarkDetails {
        num_solutions: u32,
        merkle_root: MerkleHash,
        sampled_nonces: HashSet<u64>,
    }
}
serializable_struct_with_getters! {
    BenchmarkState {
        block_confirmed: u32,
    }
}
serializable_struct_with_getters! {
    OutputMetaData {
        nonce: u64,
        runtime_signature: u64,
        fuel_consumed: u64,
        solution_signature: u64,
    }
}
impl From<OutputData> for OutputMetaData {
    fn from(data: OutputData) -> Self {
        OutputMetaData {
            solution_signature: data.calc_solution_signature(),
            runtime_signature: data.runtime_signature,
            fuel_consumed: data.fuel_consumed,
            nonce: data.nonce,
        }
    }
}
impl From<OutputMetaData> for MerkleHash {
    fn from(data: OutputMetaData) -> Self {
        MerkleHash(u8s_from_str(&jsonify(&data)))
    }
}
impl From<OutputData> for MerkleHash {
    fn from(data: OutputData) -> Self {
        MerkleHash::from(OutputMetaData::from(data))
    }
}

// Binary child structs
serializable_struct_with_getters! {
    BinaryDetails {
        compile_success: bool,
        download_url: Option<String>,
    }
}
serializable_struct_with_getters! {
    BinaryState {
        block_confirmed: u32,
    }
}

// Block child structs
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, Hash)]
#[serde(rename_all = "lowercase")]
pub enum TxType {
    Algorithm,
    Benchmark,
    Binary,
    Breakthrough,
    Challenge,
    Delegate,
    Deposit,
    Fraud,
    Precommit,
    Proof,
    RewardShare,
    Topup,
    Vote,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, Hash)]
#[serde(rename_all = "lowercase")]
pub enum SupplyType {
    Circulating,
    Locked,
    Burnt,
}
serializable_struct_with_getters! {
    BlockDetails {
        prev_block_id: String,
        height: u32,
        round: u32,
        num_confirmed: HashMap<TxType, u32>,
        num_active: HashMap<String, u32>,
        eth_block_num: Option<String>,
        supply: HashMap<SupplyType, PreciseNumber>, // circulating, locked, burnt,
        timestamp: u64,
    }
}
serializable_struct_with_getters! {
    BlockData {
        confirmed_ids: HashMap<TxType, HashSet<String>>,
        active_ids: HashMap<String, HashSet<String>>,
    }
}

// Breakthrough child structs
serializable_struct_with_getters! {
    BreakthroughDetails {
        name: String,
        player_id: String,
        challenge_id: String,
    }
}
serializable_struct_with_getters! {
    BreakthroughState {
        block_confirmed: u32,
        round_submitted: u32,
        round_pushed: Option<u32>,
        round_active: Option<u32>,
        round_merged: Option<u32>,
        vote_tally: HashMap<bool, PreciseNumber>,
    }
}
serializable_struct_with_getters! {
    BreakthroughBlockData {
        adoption: PreciseNumber,
        merge_points: u32,
        reward: PreciseNumber,
    }
}

// Challenge child structs
serializable_struct_with_getters! {
    ChallengeDetails {
        name: String,
    }
}
serializable_struct_with_getters! {
    ChallengeState {
        block_confirmed: u32,
        round_active: u32,
    }
}
serializable_struct_with_getters! {
    ChallengeBlockData {
        solution_signature_threshold: u32,
        num_qualifiers: u32,
        qualifier_difficulties: HashSet<Point>,
        base_frontier: Frontier,
        scaled_frontier: Frontier,
        scaling_factor: f64,
        base_fee: PreciseNumber,
        per_nonce_fee: PreciseNumber,
    }
}

// Delegate child structs
serializable_struct_with_getters! {
    DelegateDetails {
        player_id: String,
        delegatee: String,
    }
}
serializable_struct_with_getters! {
    DelegateState {
        block_confirmed: u32,
    }
}

// Deposit child structs
serializable_struct_with_getters! {
    DepositDetails {
        player_id: String,
        tx_hash: String,
        log_idx: u32,
        amount: PreciseNumber,
        start_timestamp: u64,
        end_timestamp: u64,
    }
}
serializable_struct_with_getters! {
    DepositState {
        block_confirmed: u32,
    }
}

// Fraud child structs
serializable_struct_with_getters! {
    FraudState {
        block_confirmed: u32,
    }
}

// OPoW child structs
serializable_struct_with_getters! {
    OPoWBlockData {
        num_qualifiers_by_challenge: HashMap<String, u32>,
        cutoff: u32,
        associated_deposit: PreciseNumber,
        delegators: HashSet<String>,
        deposit_share: PreciseNumber,
        imbalance: PreciseNumber,
        influence: PreciseNumber,
        reward: PreciseNumber,
    }
}

// Player child structs
serializable_struct_with_getters! {
    PlayerDetails {
        name: String,
        is_multisig: bool,
    }
}
serializable_struct_with_getters! {
    PlayerState {
        total_fees_paid: PreciseNumber,
        available_fee_balance: PreciseNumber,
        delegatee: String,
        votes: HashMap<String, bool>,
        reward_share: PreciseNumber,
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, Hash)]
#[serde(rename_all = "lowercase")]
pub enum RewardType {
    Benchmarker,
    Algorithm,
    Breakthrough,
    Delegator,
}
serializable_struct_with_getters! {
    PlayerBlockData {
        reward_by_type: HashMap<RewardType, PreciseNumber>,
        deposit_by_rounds: HashMap<u32, PreciseNumber>,
        weighted_deposit: PreciseNumber,
    }
}

// Precommit child structs
serializable_struct_with_getters! {
    PrecommitDetails {
        block_started: u32,
        num_nonces: u32,
        rand_hash: String,
        fee_paid: PreciseNumber,
    }
}
serializable_struct_with_getters! {
    PrecommitState {
        block_confirmed: u32,
    }
}

// Proof child structs
serializable_struct_with_getters! {
    ProofDetails {
        submission_delay: u32,
    }
}
serializable_struct_with_getters! {
    ProofState {
        block_confirmed: u32,
    }
}
pub type Solution = Map<String, Value>;
serializable_struct_with_getters! {
    OutputData {
        nonce: u64,
        runtime_signature: u64,
        fuel_consumed: u64,
        solution: Solution,
    }
}
impl OutputData {
    pub fn calc_solution_signature(&self) -> u64 {
        u64s_from_str(&jsonify(&self.solution))[0]
    }
}

// RewardShare child structs
serializable_struct_with_getters! {
    RewardShareDetails {
        player_id: String,
        share: PreciseNumber,
    }
}
serializable_struct_with_getters! {
    RewardShareState {
        block_confirmed: u32,
    }
}

// TopUp child structs
serializable_struct_with_getters! {
    TopUpDetails {
        player_id: String,
        tx_hash: String,
        log_idx: u32,
        amount: PreciseNumber,
    }
}
serializable_struct_with_getters! {
    TopUpState {
        block_confirmed: u32,
    }
}

// Vote child structs
serializable_struct_with_getters! {
    VoteDetails {
        player_id: String,
        breakthrough_id: String,
        is_breakthrough: bool,
    }
}
serializable_struct_with_getters! {
    VoteState {
        block_confirmed: u32,
    }
}
