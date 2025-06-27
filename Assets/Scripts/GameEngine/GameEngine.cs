using Ajuna.SAGE.Core;
using Ajuna.SAGE.Core.Manager;
using Ajuna.SAGE.Core.Model;
using System;
using System.Collections.Generic;
using TMPro;
using UnityEngine;

namespace SageUnityLib
{
    public class GameEngine : MonoBehaviour
    {
        [SerializeField]
        private TMP_Text _blockNumberTxt;
        public IBlockchainInfoProvider BlockchainInfoProvider { get; private set; }
        public Engine<GameIdentifier, GameRule> Engine { get; private set; }

        private void Awake()
        {
            BlockchainInfoProvider = new BlockchainInfoProvider(1234);
            var builder = new EngineBuilder<GameIdentifier, GameRule>(BlockchainInfoProvider);
            builder.SetVerifyFunction(GetVerifyFunction());

            // Register all transitions
            foreach (var (id, rules, fee, transition) in GetRulesAndTransitionSets())
            {
                builder.AddTransition(id, rules, fee, transition);
            }

            Engine = builder.Build();
        }

        private void Start()
        {
            InvokeRepeating(nameof(UpdateBlockNumber), 0f, 6f);
        }

        private void UpdateBlockNumber()
        {
            BlockchainInfoProvider.CurrentBlockNumber++;
            Debug.Log($"Blocknumber: {BlockchainInfoProvider.CurrentBlockNumber}");
            if (_blockNumberTxt != null)   
            {
                _blockNumberTxt.text = $"BLOCK: {BlockchainInfoProvider.CurrentBlockNumber}";
            }
        }

        private Func<IAccount, GameRule, IAsset[], uint, object, IBalanceManager, IAssetManager, bool>
            GetVerifyFunction()
        {
            return (account, rule, assets, balance, payload, bm, am) =>
            {
                switch (rule.RuleTypeEnum)
                {
                    case GameRuleType.IsOwnerOf:
                        if (rule.RuleValue == null || rule.RuleValue.Length == 0) return false;
                        var idx = rule.RuleValue[0];
                        if (assets.Length <= idx) return false;
                        return account.IsOwnerOf(assets[idx]);

                    default:
                        throw new NotSupportedException($"Unsupported RuleType {rule.RuleType}");
                }
            };
        }

        private static IEnumerable<(
                GameIdentifier,
                GameRule[],
                ITransitioFee,
                TransitionFunction<GameRule>)>
            GetRulesAndTransitionSets()
        {
            var result = new List<(GameIdentifier, GameRule[], ITransitioFee?, TransitionFunction<GameRule>)>
            {
                //EatTransition(),
            };
            return result;
        }

        private static (
            GameIdentifier,
            GameRule[],
            ITransitioFee,
            TransitionFunction<GameRule>)
            EatTransition()
        {
            // TODO: implement the actual transition function
            throw new NotImplementedException();
        }
    }
}