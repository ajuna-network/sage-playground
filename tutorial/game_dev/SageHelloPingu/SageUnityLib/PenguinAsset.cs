using Ajuna.SAGE.Core;
using Ajuna.SAGE.Core.Model;

namespace SageUnityLib.Model
{
    /// <summary>
    /// Penguin Player asset with health.
    /// </summary>
    public class PenguinAsset : BaseAsset
    {
        public PenguinAsset(uint ownerId, uint? initialHealth = null, uint genesis = 0)
            : base(ownerId)
        {
            AssetType = AssetType.Player;
            Health = (byte)(initialHealth ?? 95); // If null, use 10
            GenesisBlock = genesis;
        }

        public PenguinAsset(Asset existingAsset) : base(existingAsset.OwnerId)
        {
            AssetType = AssetType.Player;
            Health = existingAsset.Data.Read<byte>(1);
            GenesisBlock = existingAsset.Data.Read<uint>(2);
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

        /// <summary>
        /// Genesis block number stored at index 2.
        /// </summary>
        public uint GenesisBlock
        {
            get => Data.Read<uint>(2);
            set => Data.Set<uint>(2, value);
        }
    }
}