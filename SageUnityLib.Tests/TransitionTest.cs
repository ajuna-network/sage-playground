using Ajuna.SAGE.Core;
using Ajuna.SAGE.Core.Model;
using SageUnityLib.Model;
using NUnit.Framework;

namespace SageUnityLib.Test
{
    public class TransitionTests : BaseSetupTest
    {
        private IAccount _user;

        [SetUp]
        public void Setup()
        {
            var userId = Engine.AccountManager.Create();
            _user = Engine.AccountManager.Account(userId);
            _user.Balance.Deposit(1_000_000);
            BlockchainInfoProvider.CurrentBlockNumber++;
        }

        [Test]
        public void Test_CreatePenguin()
        {
            Assert.That(BlockchainInfoProvider.CurrentBlockNumber, Is.EqualTo(2));
            bool resultFirst = Engine.Transition(_user, GameIdentifier.CreatePenguin, null, out IAsset[] outAssets, null);
            Assert.That(resultFirst, Is.True, "transition result should succeed.");
            var penguin = outAssets[0] as PenguinAsset;
            Assert.That(penguin, Is.Not.Null);
            Assert.That(penguin.Health, Is.EqualTo(95));
        }

        [Test]
        public void Test_CreateFish()
        {
            Assert.That(BlockchainInfoProvider.CurrentBlockNumber, Is.EqualTo(2));
            bool resultFirst = Engine.Transition(_user, GameIdentifier.CreateFish, null, out IAsset[] outAssets, null);
            Assert.That(resultFirst, Is.True, "transition result should succeed.");
            var fish = outAssets[0] as FishAsset;
            Assert.That(fish, Is.Not.Null);
            Assert.That(fish.HealthValue, Is.EqualTo(5));
        }

        [Test]
        public void Test_DoEat()
        {
            Assert.That(BlockchainInfoProvider.CurrentBlockNumber, Is.EqualTo(2));
            Engine.Transition(_user, GameIdentifier.CreatePenguin, null, out _, null);
            Engine.Transition(_user, GameIdentifier.CreateFish, null, out _, null);
            bool result = Engine.Transition(_user, GameIdentifier.Eat, new IAsset[] {
                Engine.AssetManager.AssetOf(_user).First(a => ((BaseAsset)a).AssetType == AssetType.Player),
                Engine.AssetManager.AssetOf(_user).First(a => ((BaseAsset)a).AssetType == AssetType.Consumable)
            }, out IAsset[] outAssets, null);
            Assert.That(result, Is.True, "transition result should succeed.");
            Assert.That(outAssets.Length, Is.EqualTo(1), "should return one asset");
            var updatedPenguin = outAssets[0] as PenguinAsset;
            Assert.That(updatedPenguin, Is.Not.Null, "updated penguin should not be null");
            Assert.That(updatedPenguin.Health, Is.EqualTo(100), "penguin health should be 100 after eating fish");
        }
    }
}