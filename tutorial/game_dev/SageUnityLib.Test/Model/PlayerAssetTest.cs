using SageUnityLib.Model;

namespace SageUnityLib.Test.Model
{
    [TestFixture]
    public class PlayerAssetTest
    {
        private PlayerAsset playerAsset;

        [SetUp]
        public void Setup()
        {
            playerAsset = new PlayerAsset(1, 0);
        }

        [Test]
        public void Test_PlayerAsset()
        {
            Assert.That(playerAsset, Is.Not.Null);
            Assert.That(playerAsset.AssetType, Is.EqualTo(AssetType.Player));
            Assert.That(playerAsset.Health, Is.EqualTo(95), "Default health should be 95.");
        }
    }
}