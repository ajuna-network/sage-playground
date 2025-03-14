use crate::types::{AssetId, AssetType, BaseAsset};
use ajuna_primitives::sage_api::SageApi;
use sage_api::TransitionError;

pub(crate) fn ensure_asset_type_at<BlockNumber>(
	assets: &[(AssetId, BaseAsset<BlockNumber>)],
	asset_type: AssetType,
	asset_index: usize,
) -> Result<(), TransitionError> {
	let (_, asset) = assets.get(asset_index).ok_or(TransitionError::Transition { code: 0 })?;

	if asset.asset_type == asset_type {
		Ok(())
	} else {
		Err(TransitionError::Transition { code: 0 })
	}
}

fn account_has_no_asset<AccountId, BlockNumber, Sage, F>(
	account_id: &AccountId,
	filter_fn: F,
) -> bool
where
	F: Fn((AssetId, BaseAsset<BlockNumber>)) -> bool,
	Sage: SageApi<AccountId = AccountId, AssetId = AssetId, Asset = BaseAsset<BlockNumber>>,
{
	Sage::iter_assets_from(account_id).all(|asset| !filter_fn(asset))
}

pub(crate) fn ensure_account_has_no_asset_of_type<AccountId, BlockNumber, Sage>(
	account_id: &AccountId,
	asset_type: AssetType,
) -> Result<(), TransitionError>
where
	Sage: SageApi<AccountId = AccountId, AssetId = AssetId, Asset = BaseAsset<BlockNumber>>,
{
	let filter = |(_, asset): (AssetId, BaseAsset<BlockNumber>)| asset.asset_type == asset_type;

	if account_has_no_asset::<_, _, Sage, _>(account_id, filter) {
		Ok(())
	} else {
		Err(TransitionError::Transition { code: 0 })
	}
}
