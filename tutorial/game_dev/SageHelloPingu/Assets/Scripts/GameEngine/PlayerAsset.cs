using Ajuna.SAGE.Core.Model;

namespace SageUnityLib
{
    public class PlayerAsset : BaseAsset
    {
        public PlayerAsset(uint ownerId, uint score = 0, uint genesis = 0) 
            : base(ownerId, score, genesis)
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