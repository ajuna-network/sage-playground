using Ajuna.SAGE.Core.Model;

namespace SageUnityLib
{
    /// <summary>
    /// Represents a game rule used in the game engine transitions.
    /// </summary>
    public struct GameRule : ITransitionRule
    {
        public byte RuleType { get; set; }

        public byte RuleOp { get; set; }

        public byte[] RuleValue { get; set; }

        public readonly GameRuleType RuleTypeEnum => (GameRuleType)RuleType;

        public readonly GameRuleOp RuleOpEnum => (GameRuleOp)RuleOp;

        public GameRule(GameRuleType type, GameRuleOp ruleOp, byte[] ruleValue)
        {
            RuleType = (byte)type;
            RuleOp = (byte)ruleOp;
            RuleValue = ruleValue;
        }
    }
}