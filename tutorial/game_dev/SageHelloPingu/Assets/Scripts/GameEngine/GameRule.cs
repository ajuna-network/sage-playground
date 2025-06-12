using Ajuna.SAGE.Core.Model;
using System;

namespace SageUnityLib
{
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