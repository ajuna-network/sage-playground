using Ajuna.SAGE.Core.Model;
using SageUnityLib;

namespace SageUnityLib.Model
{
    public static class GameConfig
    {
        internal static GameIdentifier CreatePenguin(out GameRule[] rules, out ITransitioFee fee)
        {
            rules = new GameRule[] { };
            fee = default;
            return new GameIdentifier((byte)GameAction.CreatePenguin);
        }

        internal static GameIdentifier CreateFish(out GameRule[] rules, out ITransitioFee fee)
        {
            rules = new GameRule[] { };
            fee = default;
            return new GameIdentifier((byte)GameAction.CreateFish);
        }

        internal static GameIdentifier Eat(out GameRule[] rules, out ITransitioFee fee)
        {
            byte playerAsset = (byte)AssetType.Player;
            byte fishAsset = (byte)AssetType.Consumable;

            rules = new[]
            {
            // Validate ownership of both assets
            new GameRule(GameRuleType.IsOwnerOf, GameRuleOp.Index, new byte[] { 0x00 }),
            new GameRule(GameRuleType.IsOwnerOf, GameRuleOp.Index, new byte[] { 0x01 }),
            // Ensure types: first Player, then Consumable
            new GameRule(GameRuleType.AssetTypesAt, GameRuleOp.Composite, new byte[] { playerAsset, fishAsset })
        };

            fee = default; // no fee for this demo

            return new GameIdentifier((byte)GameAction.Eat);
        }
    }
}