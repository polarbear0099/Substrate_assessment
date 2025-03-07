use frame_support::{parameter_types, traits::Everything};
use sp_runtime::traits::ConvertInto;
// Import the EVM pallet
use pallet_evm::{Account as EVMAccount, Config, FeeCalculator};

parameter_types! {
    pub const ExistentialDeposit: u128 = 500;
    pub const MaxLocks: u32 = 50;
	pub const ChainId: u64 = 42; // Chain ID for your blockchain
    pub const BlockGasLimit: u32 = 10_000_000; // Gas limit per block
}

impl pallet_balances::Config for Runtime {
    type MaxLocks = MaxLocks;
    type Balance = u128;
    type DustRemoval = ();
    type Event = Event;
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
}

impl pallet_staking::Config for Runtime {
    type Currency = Balances;
    type UnixTime = Timestamp;
    type CurrencyToVote = ConvertInto;
    type Reward = ();
    type Event = Event;
    type Slash = ();
    type RewardRemainder = ();
    type SessionsPerEra = ();
    type SlashDeferDuration = ();
    type SlashCancelOrigin = frame_system::EnsureRoot<AccountId>;
    type BondingDuration = ();
    type SessionInterface = Self;
    type EraPayout = ();
    type NextNewSession = Session;
    type MaxNominatorRewardedPerValidator = ();
    type HistoryDepth = ();
    type WeightInfo = ();
}