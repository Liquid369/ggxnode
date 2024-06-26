use crate as pallet_dex;

use frame_support::{
	pallet_prelude::Weight,
	parameter_types, sp_io,
	traits::{AsEnsureOriginWithArg, GenesisBuild, Hooks},
	weights::constants::RocksDbWeight,
	PalletId,
};
use sp_core::{ConstU128, ConstU32, ConstU64, H256};
use sp_runtime::{testing::Header, traits::IdentityLookup};

pub type AccountId = u128;
pub type Balance = u128;
pub type AssetId = u32;
pub type BlockNumber = u64;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system,
		Balances: pallet_balances,
		Timestamp: pallet_timestamp,
	Assets: pallet_assets,
	Dex: pallet_dex,
	}
);

impl pallet_balances::Config for Test {
	type Balance = Balance;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ConstU128<1>;
	type AccountStore = System;
	type WeightInfo = pallet_balances::weights::SubstrateWeight<Test>;
	type MaxLocks = ConstU32<50>;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type HoldIdentifier = ();
	type FreezeIdentifier = ();
	type MaxHolds = ConstU32<0>;
	type MaxFreezes = ConstU32<0>;
}

// These parameters dont matter much as this will only be called by root with the forced arguments
// No deposit is substracted with those methods
parameter_types! {
  pub const AssetDeposit: Balance = 0;
  pub const AssetAccountDeposit: Balance = 0;
  pub const ApprovalDeposit: Balance = 0;
  pub const AssetsStringLimit: u32 = 50;
  pub const MetadataDepositBase: Balance = 0;
  pub const MetadataDepositPerByte: Balance = 0;
}

impl pallet_assets::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type AssetId = AssetId;
	type Currency = Balances;
	type ForceOrigin = frame_system::EnsureRoot<AccountId>;
	type AssetDeposit = AssetDeposit;
	type AssetAccountDeposit = AssetAccountDeposit;
	type MetadataDepositBase = MetadataDepositBase;
	type MetadataDepositPerByte = MetadataDepositPerByte;
	type ApprovalDeposit = ApprovalDeposit;
	type StringLimit = AssetsStringLimit;
	type Freezer = ();
	type Extra = ();
	type CreateOrigin = AsEnsureOriginWithArg<frame_system::EnsureSigned<AccountId>>;
	type WeightInfo = pallet_assets::weights::SubstrateWeight<Test>;
	type RemoveItemsLimit = ConstU32<0>;
	type AssetIdParameter = AssetId;
	type CallbackHandle = ();
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = ();
}

parameter_types! {
	pub BlockWeights: frame_system::limits::BlockWeights =
			frame_system::limits::BlockWeights::simple_max(
				Weight::from_parts(2_000_000_000_000, u64::MAX),
			);
}

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = BlockWeights;
	type BlockLength = ();
	type DbWeight = RocksDbWeight;
	type RuntimeOrigin = RuntimeOrigin;
	type Index = u64;
	type BlockNumber = u64;
	type RuntimeCall = RuntimeCall;
	type Hash = H256;
	type Version = ();
	type Hashing = sp_runtime::traits::BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl<LocalCall> frame_system::offchain::SendTransactionTypes<LocalCall> for Test
where
	RuntimeCall: From<LocalCall>,
{
	type OverarchingCall = RuntimeCall;
	type Extrinsic = Extrinsic;
}
pub type Extrinsic = sp_runtime::testing::TestXt<RuntimeCall, ()>;

impl pallet_timestamp::Config for Test {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = ConstU64<2>;
	type WeightInfo = ();
}

parameter_types! {
	pub const DexPalletId: PalletId = PalletId(*b"py/sudex");
	pub const UnsignedPriority: BlockNumber = 1;
}

impl pallet_dex::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type PalletId = DexPalletId;
	type Fungibles = Assets;
	type PrivilegedOrigin = frame_system::EnsureRoot<Self::AccountId>;
	type Currency = Balances;
	type UnsignedPriority = UnsignedPriority;
}

pub struct ExtBuilder;

impl Default for ExtBuilder {
	fn default() -> Self {
		ExtBuilder
	}
}

impl ExtBuilder {
	pub fn build(self) -> sp_io::TestExternalities {
		let mut storage = frame_system::GenesisConfig::default()
			.build_storage::<Test>()
			.unwrap();

		// This will cause some initial issuance
		pallet_balances::GenesisConfig::<Test> {
			balances: vec![(1, 9000), (2, 800)],
		}
		.assimilate_storage(&mut storage)
		.ok();

		pallet_assets::GenesisConfig::<Test> {
			assets: vec![
				// id, owner, is_sufficient, min_balance
				(999, 0, true, 1),
				(888, 0, true, 1),
				(777, 0, true, 1),
			],
			metadata: vec![
				// id, name, symbol, decimals
				(999, "Bitcoin".into(), "BTC".into(), 8),
				(888, "GGxchain".into(), "GGXT".into(), 18),
				(777, "USDT".into(), "USDT".into(), 6),
			],
			accounts: vec![
				// id, account_id, balance
				(999, 1, 1_000_000_000),
				(888, 1, 1_000_000_000),
				(777, 1, 1_000_000_000),
				(999, 2, 1_000_000_000),
				(888, 2, 1_000_000_000),
				(777, 2, 1_000_000_000),
			],
		}
		.assimilate_storage(&mut storage)
		.ok();

		<pallet_dex::GenesisConfig as frame_support::traits::GenesisBuild<Test>>::assimilate_storage(
      &pallet_dex::GenesisConfig {
        asset_ids: vec![8888, 999, 888, 777],
        native_asset_id: 8888,
      },
      &mut storage,
  )
  .unwrap();

		let mut ext = sp_io::TestExternalities::new(storage);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	ExtBuilder::default().build()
}

pub fn run_to_block(n: u64) {
	while System::block_number() < n {
		if System::block_number() > 0 {
			Dex::on_finalize(System::block_number());
			System::on_finalize(System::block_number());
		}
		System::reset_events();
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
		Dex::on_initialize(System::block_number());
	}
}

fn new_block() -> Weight {
	let number = frame_system::Pallet::<Test>::block_number() + 1;
	let hash = H256::repeat_byte(number as u8);

	frame_system::Pallet::<Test>::reset_events();
	frame_system::Pallet::<Test>::initialize(&number, &hash, &Default::default());

	Weight::default()
}

pub fn add_blocks(blocks: usize) {
	for _ in 0..blocks {
		new_block();
	}
}
