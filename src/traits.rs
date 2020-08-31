use frame_support::{
    dispatch::{result::Result, DispatchError, DispatchResult},
    traits::Get,
};

pub trait Nft {
    /// The type used to identify unique assets.
    type Id;
    /// The attributes that distinguish unique assets.
    type Info;
}

pub trait UniqueAssets<Asset: Nft> {
    /// The type used to identify asset owners.
    type AccountId;

    // TODO: Should not be part of unique assets, make a Cappable trait
    type AssetLimit: Get<u128>;
    type UserAssetLimit: Get<u64>;


    /// The total number of this type of asset that exists (minted - burned).
    fn total() -> u128;
    /// The total number of this type of asset owned by an account.
    fn total_for_account(account: &Self::AccountId) -> u64;
    /// The set of unique assets owned by an account.
    fn assets_for_account(account: &Self::AccountId) -> Vec<Asset>;
    /// The ID of the account that owns an asset.
    fn owner_of(asset_id: &Asset::Id) -> Self::AccountId;

    /// Transfer ownership of an asset to another account.
    /// This method **must** return an error in the following cases:
    /// - The asset with the specified ID does not exist.
    /// - The destination account has already reached the user asset limit.
    fn transfer(dest_account: &Self::AccountId, asset_id: &Asset::Id) -> DispatchResult;
}

pub trait Mintable<Asset: Nft> {
    /// The type used to identify asset owners.
    type AccountId;

    /// Use the provided asset info to create a new unique asset for the specified user.
    /// This method **must** return an error in the following cases:
    /// - The asset, as identified by the asset info, already exists.
    /// - The specified owner account has already reached the user asset limit.
    /// - The total asset limit has already been reached.
    fn mint(owner: &Self::AccountId,
            asset_info: Asset::Info,
    ) -> Result<Asset::Id, DispatchError>;
}

pub trait Burnable<Asset: Nft> {

    /// Destroy an asset.
    /// This method **must** return an error in the following case:
    /// - The asset with the specified ID does not exist.
    fn burn(asset_id: &Asset::Id) -> DispatchResult;
    /// The total number of this type of asset that has been burned (may overflow).
    fn burned() -> u128;
}
