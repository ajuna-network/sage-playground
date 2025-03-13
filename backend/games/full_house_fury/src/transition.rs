use crate::{
    error::*,
};

use ajuna_primitives::{payment_handler::NativeId, sage_api::SageApi};
use sage_api::{rules::*, traits::TransitionOutput, SageGameTransition, TransitionError};

use frame_support::{
    pallet_prelude::{Decode, Encode, TypeInfo},
    Parameter,
};
use parity_scale_codec::{Codec, MaxEncodedLen};
use sp_core::H256;
use sp_runtime::{
    traits::{AtLeast32BitUnsigned, BlockNumber as BlockNumberT, Member},
    SaturatedConversion,
};
use sp_std::{marker::PhantomData, vec::Vec};
use crate::types::{AssetId, BaseAsset};

pub type TransitionConfig = ();

pub struct FullHouseFuryTransition<AccountId, BlockNumber, Sage> {
    _phantom: PhantomData<(AccountId, BlockNumber, Sage)>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub enum TransitionIdentifier {
    Start,
    Play,
    Preparation,
    Battle,
    Discard,
    Score,
}


impl<AccountId, BlockNumber, Balance, Sage> FullHouseFuryTransition<AccountId, BlockNumber, Sage>
where
    AccountId: Member + Codec,
    BlockNumber: BlockNumberT,
    Balance: Member + Parameter + AtLeast32BitUnsigned + MaxEncodedLen,
    Sage: SageApi<
        AccountId = AccountId,
        AssetId = AssetId,
        Asset = BaseAsset<BlockNumber>,
        Balance = Balance,
        BlockNumber = BlockNumber,
        TransitionConfig = TransitionConfig,
        HashOutput = H256,
    >,
{
    fn try_get_asset(asset_id: &AssetId) -> Result<BaseAsset<BlockNumber>, TransitionError> {
        let asset = Sage::get_asset(asset_id)
            // Fixme, this kind of error should be in handled in the api
            .map_err(|_| TransitionError::Transition { code: 0 })?;

        Ok(asset)
    }

    fn try_get_assets(
        asset_ids: &[AssetId],
    ) -> Result<Vec<(AssetId, BaseAsset<BlockNumber>)>, TransitionError> {
        asset_ids
            .iter()
            .copied()
            .map(|asset_id| Ok((asset_id, Self::try_get_asset(&asset_id)?)))
            .collect::<Result<Vec<_>, _>>()
    }

    fn get_asset_funds(
        asset_id: &AssetId,
        payment_asset: Option<&Sage::FungiblesAssetId>,
    ) -> Balance {
        let fund_id = if let Some(payment) = payment_asset {
            payment
        } else {
            &Sage::FungiblesAssetId::get_native_id()
        };

        Sage::inspect_asset_funds(asset_id, fund_id)
    }

    fn deposit_funds_to_asset(
        asset_id: &AssetId,
        from: &AccountId,
        amount: Balance,
    ) -> Result<(), TransitionError> {
        let fund_id = Sage::FungiblesAssetId::get_native_id();
        Sage::deposit_funds_to_asset(asset_id, from, fund_id, amount)
            .map_err(|_| TransitionError::Transition { code: 0 })
    }

    fn withdraw_funds_from_asset(
        asset_id: &AssetId,
        to: &AccountId,
        amount: Balance,
    ) -> Result<(), TransitionError> {
        let fund_id = Sage::FungiblesAssetId::get_native_id();
        Sage::transfer_funds_from_asset(asset_id, to, fund_id, amount)
            .map_err(|_| TransitionError::Transition { code: 0 })
    }

    fn generate_asset_id() -> Result<AssetId, TransitionError> {
        Sage::create_next_asset_id().ok_or(TransitionError::CouldNotCreateAssetId)
    }

    fn verify_transition_rules(
        transition_id: &TransitionIdentifier,
        account_id: &AccountId,
        asset_ids: &[AssetId],
    ) -> Result<Vec<(AssetId, BaseAsset<BlockNumber>)>, TransitionError> {
        let mut maybe_assets = None;

        match transition_id {
            TransitionIdentifier::Start => {
                ensure_asset_length(asset_ids, 0)?;

            },
            _ => {}
        }

        if maybe_assets.is_none() {
            maybe_assets = Some(Self::try_get_assets(asset_ids)?);
        }

        Ok(maybe_assets.unwrap())
    }

    fn transition_assets(
        transition_id: &TransitionIdentifier,
        account_id: &AccountId,
        mut assets: Vec<(AssetId, BaseAsset<BlockNumber>)>,
        payment_asset: Option<Sage::FungiblesAssetId>,
    ) -> Result<Vec<TransitionOutput<AssetId, BaseAsset<BlockNumber>>>, TransitionError> {
        let output = match transition_id {
            TransitionIdentifier::Start => {
                let current_block = Sage::get_current_block_number();
                }
            _ => {}
        };

        Ok(Default::default())
    }
}

impl<AccountId, BlockNumber, Balance, Sage> SageGameTransition
for FullHouseFuryTransition<AccountId, BlockNumber, Sage>
where
    AccountId: Member + Codec,
    BlockNumber: BlockNumberT,
    Balance: Member + Parameter + AtLeast32BitUnsigned + MaxEncodedLen,
    Sage: SageApi<
        AccountId = AccountId,
        AssetId = AssetId,
        Asset = BaseAsset<BlockNumber>,
        Balance = Balance,
        BlockNumber = BlockNumber,
        TransitionConfig = TransitionConfig,
        HashOutput = H256,
    >,
{
    type TransitionId = TransitionIdentifier;
    type TransitionConfig = TransitionConfig;
    type AccountId = AccountId;
    type AssetId = AssetId;
    type Asset = BaseAsset<BlockNumber>;
    type Extra = ();
    type PaymentFungible = Sage::FungiblesAssetId;

    fn do_transition(
        transition_id: &Self::TransitionId,
        account_id: &Self::AccountId,
        assets_ids: &[Self::AssetId],
        _: &Self::Extra,
        payment_asset: Option<Self::PaymentFungible>,
    ) -> Result<Vec<TransitionOutput<Self::AssetId, Self::Asset>>, TransitionError> {
        let assets = Self::verify_transition_rules(transition_id, account_id, assets_ids)?;
        Self::transition_assets(transition_id, account_id, assets, payment_asset)
    }
}
