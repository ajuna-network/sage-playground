// Ajuna Node
// Copyright (C) 2022 BlogaTech AG

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

mod battle_mogs;
mod casino_jam;

mod fee_handler {
	use crate::{AccountId, Balance, Balances};

	use ajuna_payment_handler::{
		AllowAllAssets, AssetGameFeeHandler, VoucherHandler, WithdrawCreditOrVoucher,
		WithdrawFungibles, WithdrawWhitelistedCredit,
	};

	use frame_support::traits::fungible::{NativeFromLeft, NativeOrWithId, UnionOf};
	use sp_runtime::DispatchError;

	pub type NativeAndAssets<Assets, AssetId> =
		UnionOf<Balances, Assets, NativeFromLeft, NativeOrWithId<AssetId>, AccountId>;

	/// Fee handler facilitating payment in the native currency and with whitelisted types.
	pub type SageFeeHandler<
		Assets,
		AssetId,
		Affiliates,
		AffiliateMaxLevel,
		Tournament,
		VoucherHandler,
	> = AssetGameFeeHandler<
		AccountId,
		NativeAndAssets<Assets, AssetId>,
		WithdrawCreditOrVoucher<
			WithdrawWhitelistedCredit<
				AllowAllAssets<NativeOrWithId<AssetId>>,
				WithdrawFungibles<AccountId, NativeAndAssets<Assets, AssetId>>,
			>,
			VoucherHandler,
		>,
		Affiliates,
		AffiliateMaxLevel,
		Tournament,
	>;

	/*pub type FungiblesAssetId = WithdrawKind<NativeOrWithId<SageAssetId>>;
	pub type SageNativeAndAssets = NativeAndAssets<SageAssets, SageAssetId>;
	pub type TransferWithdraw =
	WithdrawCreditOrVoucher<WithdrawFungibles<AccountId, SageNativeAndAssets>, DummyVoucherHandler>;*/

	pub struct DummyVoucherHandler;

	impl VoucherHandler for DummyVoucherHandler {
		type AccountId = AccountId;
		type Balance = Balance;

		fn consume_vouchers_from(
			_account: &Self::AccountId,
			_amount: Self::Balance,
		) -> Result<(), DispatchError> {
			Ok(())
		}
	}
}
