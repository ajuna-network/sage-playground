namespace SageUnityLib.Model
{
    public enum AssetType : byte
    {
        None = 0,
        Player = 1,
        Consumable = 2,
    }

    public enum GameAction : byte
    {
        None = 0,
        CreatePenguin = 1,
        CreateFish = 2,
        Eat = 3,  // moved from 1 to 3
    }

    public enum GameRuleType : byte
    {
        None = 0,
        IsOwnerOf = 1,
        AssetTypesAt = 2,
    }

    public enum GameRuleOp : byte
    {
        None = 0,
        Index = 1,
        Composite = 2,
    }
}