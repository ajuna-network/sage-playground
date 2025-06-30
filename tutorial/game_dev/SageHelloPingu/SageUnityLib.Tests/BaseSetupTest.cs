using Ajuna.SAGE.Core;
using Ajuna.SAGE.Core.Model;
using SageUnityLib.Model;
using NUnit.Framework;

namespace SageUnityLib.Test
{
    public class BaseSetupTest
    {
        public IBlockchainInfoProvider BlockchainInfoProvider { get; private set; }
        public Engine<GameIdentifier, GameRule> Engine { get; private set; }

        [SetUp]
        public void Setup()
        {
            BlockchainInfoProvider = new BlockchainInfoProvider(1234);
            Engine = GameEngine.Create(BlockchainInfoProvider);
        }

        public void Reset()
        {
            BlockchainInfoProvider = new BlockchainInfoProvider(1234);
            Engine = GameEngine.Create(BlockchainInfoProvider);
        }

        public T GetAsset<T>(IAccount user, AssetType type) where T : BaseAsset
        {
            var result = Engine.AssetManager
                .AssetOf(user)
                .OfType<BaseAsset>()
                .FirstOrDefault(p => p.AssetType == type);
            Assert.That(result, Is.Not.Null);
            Assert.That(result, Is.InstanceOf<T>());
            return (T)result;
        }
    }
}