using Ajuna.SAGE.Core.Model;

namespace SageUnityLib.Model
{
    public class ConsumableAsset : BaseAsset
    {
        /// <summary>
        /// Constructs a Fish with a specified health value.
        /// </summary>
        public ConsumableAsset(uint ownerId, uint score = 0, uint genesis = 0)
            : base(ownerId, score, genesis)
        {
            AssetType = AssetType.Consumable;
            HealthValue = 5;
        }

        public ConsumableAsset(IAsset asset)
            : base(asset) { }

        /// <summary>
        /// Health value returned on consumation of the Asset.
        /// Stored as a byte allowing a range of 0 - 255.
        /// </summary>
        public byte HealthValue
        {
            get => Data.Read<byte>(1);
            set => Data.Set(1, value);
        }
    }
}