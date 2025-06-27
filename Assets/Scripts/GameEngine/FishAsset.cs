using Ajuna.SAGE.Core;
using Ajuna.SAGE.Core.Model;

namespace SageUnityLib
{
  /// <summary>
  /// Fish consumable asset that restores health.
  /// </summary>
  public class FishAsset : BaseAsset
  {
    /// <summary>
    /// Constructs a Fish with a specified health value.
    /// </summary>
    public FishAsset(uint ownerId, uint healthValue = 5)
        : base(ownerId)
    {
      AssetType = AssetType.Consumable;
      HealthValue = 5;
    }

    /// <summary>
    /// Constructs a Fish from an existing Asset object.
    /// </summary>
    public FishAsset(Asset existingAsset) : base(existingAsset.OwnerId)
    {
      AssetType = AssetType.Consumable;
      HealthValue = existingAsset.Data.Read<byte>(1);
    }

    /// <summary>
    /// Current health returned on consumation of the Asset.
    /// Stored as a byte allowing a range of 0 - 255.
    /// </summary>
    public byte HealthValue
    {
      get => Data.Read<byte>(1);
      set => Data.Set(1, value);
    }
  }
}