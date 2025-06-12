using Ajuna.SAGE.Core;
using Ajuna.SAGE.Core.Model;

namespace SageUnityLib
{

    public class BaseAsset : Asset
    {
        public BaseAsset(uint ownerId, uint score = 0, uint genesis = 0) 
            : base(Utils.GenerateRandomId(), ownerId, 1, score, genesis, new byte[32])
        {
        }

        public AssetType AssetType
        {
            get => (AssetType)Data.Read<byte>(0);
            set => Data.Set<byte>(0, (byte)value);
        }

    }
}