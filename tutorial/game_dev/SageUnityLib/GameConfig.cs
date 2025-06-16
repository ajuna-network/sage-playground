using Ajuna.SAGE.Core.Model;

namespace SageUnityLib
{
    /// <summary>
    /// Configuration class for the game, defining actions and rules.
    /// </summary>
    public class GameConfig
    {
        /// <summary>
        /// Creates a GameIdentifier for creating a Penguin asset.
        /// </summary>
        /// <param name="rules"></param>
        /// <param name="fee"></param>
        /// <returns></returns>
        public static GameIdentifier CreatePenguin(out GameRule[] rules, out ITransitioFee? fee)
        {
            rules = new GameRule[] { };
            fee = default;
            return new GameIdentifier((byte)GameAction.CreatePenguin);
        }

        /// <summary>
        /// Creates a GameIdentifier for creating a Fish asset.
        /// </summary>
        /// <param name="rules"></param>
        /// <param name="fee"></param>
        /// <returns></returns>
        public static GameIdentifier CreateFish(out GameRule[] rules, out ITransitioFee? fee)
        {
            rules = new GameRule[] { };
            fee = default;
            return new GameIdentifier((byte)GameAction.CreateFish);
        }

        /// <summary>
        /// Creates a GameIdentifier for the Eat action, which consumes a Fish asset.
        /// </summary>
        /// <param name="rules"></param>
        /// <param name="fee"></param>
        /// <returns></returns>
        public static GameIdentifier DoEat(out GameRule[] rules, out ITransitioFee? fee)
        {
            byte playerAsset = (byte)AssetType.Player;
            byte fishAsset = (byte)AssetType.Consumable;

            rules = new GameRule[] {
                new GameRule(GameRuleType.IsOwnerOf, GameRuleOp.Index, new byte[] { 0x00}),
                new GameRule(GameRuleType.IsOwnerOf, GameRuleOp.Index, new byte[] { 0x01}),
                new GameRule(GameRuleType.AssetTypesAt, GameRuleOp.Composite, new byte[] { playerAsset, fishAsset }),
            };

            fee = default;

            return new GameIdentifier((byte)GameAction.DoEat);
        }
    }
}