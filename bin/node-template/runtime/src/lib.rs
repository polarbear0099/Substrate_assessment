use frame_support::{parameter_types, traits::Everything};
use sp_runtime::traits::ConvertInto;
// Import the EVM pallet
use pallet_evm::{Account as EVMAccount, Config, FeeCalculator};
use pallet_staking:: {EraIndex, SessionIndex};

parameter_types! {
    pub const ExistentialDeposit: u128 = 500;
    pub const MaxLocks: u32 = 50;

	// Define the necessary parameters for the EVM wallet
	pub const ChainId: u64 = 42; // Chain ID for your blockchain
    pub const BlockGasLimit: u32 = 10_000_000; // Gas limit per block
    
	// Define parameters for the staking pallet
	pub const SessionsPerEra: SessionIndex = 6; // Number of sessions per era
    pub const BondingDuration: EraIndex = 24 * 28; // Bonding duration in eras
    pub const SlashDeferDuration: EraIndex = 24 * 7; // Slash defer duration in eras
    pub const MaxNominators: u32 = 1000; // Maximum number of nominators
    pub const MaxValidators: u32 = 100; // Maximum number of validators

	// Define parameters for transaction payment
	pub const TransactionByteFee: u128 = 1; // Fee per byte of transaction
    pub const WeightToFee: u128 = 1; // Conversion factor from weight to fee	

	// Define targeted fee adjustment parameters
	pub const TargetedFeeAdjustmentTolerance: FixedU128 = FixedU128::from_rational(1, 100); // 1% tolerance for fee adjustment
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

// Implement the EVM pallet
impl Config for Runtime {
	type FeeCalculator = FixedGasPrice; // Fixed gas price for simplicity
    type GasWeightMapping = (); // No custom gas weight mapping
    type CallOrigin = EnsureAddressRoot<AccountId>; // Origin for EVM calls
    type Event = Event; // Event type
    type Precompiles = (); // No precompiles
    type ChainId = ChainId; // Chain ID
    type BlockGasLimit = BlockGasLimit; // Block gas limit
    type OnChargeTransaction = (); // No custom transaction charging logic
}

// Implement the transaction payment pallet configuration for your runtime
impl pallet_transaction_payment::Config for Runtime {
    type OnChargeTransaction = pallet_transaction_payment::CurrencyAdapter<Balances, ()>; // Currency adapter for fee handling
    type TransactionByteFee = TransactionByteFee; // Fee per byte
    type WeightToFee = IdentityFee<Balance>; // Identity fee calculation (1:1 weight to fee)
    type FeeMultiplierUpdate = TargetedFeeAdjustment<Self::BlockNumber, TargetedFeeAdjustmentTolerance>; // Dynamic fee adjustment
}

// Implement the fee multiplier update logic
pub struct TargetedFeeAdjustment<BlockNumber, Tolerance>(sp_std::marker::PhantomData<(BlockNumber, Tolerance)>);

impl<BlockNumber, Tolerance> MultiplierUpdate for TargetedFeeAdjustment<BlockNumber, Tolerance>
where
    BlockNumber: AtLeast32BitUnsigned + Copy,
    Tolerance: Get<FixedU128>,
{
    fn update_multiplier(
        current_multiplier: FixedU128,
        previous_weight: Weight,
        target_weight: Weight,
        block_number: BlockNumber,
    ) -> FixedU128 {
        // Implement your dynamic fee adjustment logic here
        // Example: Adjust the fee multiplier based on network congestion
        let tolerance = Tolerance::get();
        let adjustment = if previous_weight > target_weight {
            current_multiplier.saturating_mul(tolerance)
        } else {
            current_multiplier.saturating_div(tolerance)
        };
        adjustment
    }
}

// Add the EVM pallet to your runtime
construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        // Other pallets...
        EVM: pallet_evm::{Module, Call, Storage, Config, Event<T>},
		Staking: pallet_staking::{Module, Call, Storage, Config<T>, Event<T>},
		TransactionPayment: pallet_transaction_payment::{Module, Call, Storage, Config<T>},
	}
);