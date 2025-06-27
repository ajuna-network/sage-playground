using Ajuna.SAGE.Core.Model;
using SageUnityLib;

public static class GameConfig
{
    internal static GameIdentifier Eat(out GameRule[] rules, out ITransitioFee fee)
    {
        byte playerAsset = (byte)AssetType.Player;
        byte fishAsset  = (byte)AssetType.Consumable;

        rules = new [] {
            // Validate ownership of both assets
            new GameRule(GameRuleType.IsOwnerOf, GameRuleOp.Index, new byte[]{ 0x00 }),
            new GameRule(GameRuleType.IsOwnerOf, GameRuleOp.Index, new byte[]{ 0x01 }),
            // Ensure types: first Player, then Consumable
            new GameRule(GameRuleType.AssetTypesAt, GameRuleOp.Composite, new byte[]{ playerAsset, fishAsset })
        };

        fee = default; // no fee for this demo

        return new GameIdentifier((byte)GameAction.Eat);
    }
}