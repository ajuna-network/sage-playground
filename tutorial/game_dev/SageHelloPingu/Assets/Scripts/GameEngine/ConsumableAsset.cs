using Ajuna.SAGE.Core.Model;
using UnityEngine;
namespace SageUnityLib
{
    public class ConsumableAsset : BaseAsset
    {
        /// <summary>
        /// Constructs a Fish with a specified health value.
        /// </summary>
        public ConsumableAsset(uint ownerId)
            : base(ownerId)
        {
            AssetType = AssetType.Consumable;
            HealthValue = 5;
        }

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