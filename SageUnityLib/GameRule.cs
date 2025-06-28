using Ajuna.SAGE.Core.Model;
using System;

namespace SageUnityLib.Model
{
    public struct GameRule : ITransitionRule
    {
        public byte RuleType { get; set; }
        public byte RuleOp { get; set; }
        public byte[] RuleValue { get; set; }

        /// <summary>
        /// Helper to interpret the rule type.
        /// </summary>
        public GameRuleType RuleTypeEnum => (GameRuleType)RuleType;

        /// <summary>
        /// Helper to interpret the rule operator.
        /// </summary>
        public GameRuleOp RuleOpEnum => (GameRuleOp)RuleOp;

        /// <summary>
        /// Constructs a rule with specified type, operator, and value.
        /// </summary>
        public GameRule(GameRuleType type, GameRuleOp ruleOp, byte[] ruleValue)
        {
            RuleType = (byte)type;
            RuleOp = (byte)ruleOp;
            RuleValue = ruleValue;
        }
    }
}