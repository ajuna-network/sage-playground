using Ajuna.SAGE.Core;
using Ajuna.SAGE.Core.Model;
using SageUnityLib.Model;

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

        /// <summary>
        /// Resets the test environment by reinitializing the blockchain info provider and game engine.
        /// </summary>
        public void Reset()
        {
            BlockchainInfoProvider = new BlockchainInfoProvider(1234);
            Engine = GameEngine.Create(BlockchainInfoProvider);
        }

        /// <summary>
        /// Gets the asset of the specified type for the given user.
        /// </summary>
        /// <typeparam name="T"></typeparam>
        /// <param name="user"></param>
        /// <param name="type"></param>
        /// <returns></returns>
        public T GetAsset<T>(IAccount user, AssetType type) where T : BaseAsset
        {
            BaseAsset? result = Engine.AssetManager
                .AssetOf(user)
                .Select(p => (BaseAsset)p)
                .Where(p => p.AssetType == type)
                .FirstOrDefault();
            Assert.That(result, Is.Not.Null);
            Assert.That(result, Is.InstanceOf<T>());
            var typedResult = result as T;
            Assert.That(typedResult, Is.Not.Null);
            return typedResult;
        }
    }
}