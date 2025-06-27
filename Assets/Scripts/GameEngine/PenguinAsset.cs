using Ajuna.SAGE.Core;
using Ajuna.SAGE.Core.Model;
using UnityEngine;

namespace SageUnityLib
{
  /// <summary>
  /// Penguin Player asset with health.
  /// </summary>
  public class PenguinAsset : BaseAsset
  {
    public PenguinAsset(uint ownerId, uint initialHealth = 10)
        : base(ownerId)
    {
      AssetType = AssetType.Player;
      Health = 100;
    }

    /// <summary>
    /// Current health of the Penguin.
    /// Stored as a byte allowing a range of 0 - 255.
    /// </summary>
    public byte Health
    {
      get => Data.Read<byte>(1);
      set => Data.Set<byte>(1, value);
    }
  }
}