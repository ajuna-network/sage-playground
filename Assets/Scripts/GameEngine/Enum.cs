namespace SageUnityLib
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
        Eat = 1,
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