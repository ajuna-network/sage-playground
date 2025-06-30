using SageUnityLib.Model;

namespace SageUnityLib.Test.Model
{
    [TestFixture]
    public class ConsumableTest
    {
        private FishAsset consumableAsset;

        [SetUp]
        public void Setup()
        {
            consumableAsset = new FishAsset(1, null, 0);
        }

        [Test]
        public void Test_ConsumableAsset()
        {
            Assert.That(consumableAsset, Is.Not.Null);
            Assert.That(consumableAsset.AssetType, Is.EqualTo(AssetType.Consumable));
            Assert.That(consumableAsset.HealthValue, Is.EqualTo(5), "Default healthValue should be 5.");
        }
    }
}