using Ajuna.SAGE.Core;
using Ajuna.SAGE.Core.Manager;
using Ajuna.SAGE.Core.Model;
using SageUnityLib;
using System;
using System.Collections.Generic;
using System.Linq;
using TMPro;
using UnityEngine;

public class GameEngine : MonoBehaviour
{
    public IBlockchainInfoProvider BlockchainInfoProvider { get; private set; }
    public Engine<GameIdentifier, GameRule> Engine { get; private set; }

    [SerializeField]
    private TMP_Text _blockNumberTxt;

    private void Awake()
    {
        BlockchainInfoProvider = new BlockchainInfoProvider(1234);
        var engineBuilder = new EngineBuilder<GameIdentifier, GameRule>(BlockchainInfoProvider);
        engineBuilder.SetVerifyFunction(GetVerifyFunction());
        var rulesAndTransitions = GetRulesAndTranstionSets();
        foreach (var (identifier, rules, fee, transition) in rulesAndTransitions)
        {
            engineBuilder.AddTransition(identifier, rules, fee, transition);
        }

        Engine = engineBuilder.Build();
    }

    private void Start()
    {
        // update block number
        InvokeRepeating(nameof(UpdatedBlocknumber), 0f, 6f);
    }

    // Update is called once per frame
    private void Update()
    {
    }

    private void UpdatedBlocknumber()
    {
        BlockchainInfoProvider.CurrentBlockNumber++;
        Debug.Log($"Blocknumber: {BlockchainInfoProvider.CurrentBlockNumber}");
        if (_blockNumberTxt != null)   
        {
            _blockNumberTxt.text = $"BLOCK: {BlockchainInfoProvider.CurrentBlockNumber}";
        }
    }

    private Func<IAccount, GameRule, IAsset[], uint, object, IBalanceManager, IAssetManager, bool> GetVerifyFunction()
    {
        return (p, r, a, b, c, m, s) =>
        {
            switch (r.RuleTypeEnum)
            {
                case GameRuleType.IsOwnerOf:
                    {
                        if (r.RuleOpEnum != GameRuleOp.Index)
                        {
                            return false;
                        }

                        if (r.RuleValue == null || r.RuleValue.Length == 0)
                        {
                            return false;
                        }

                        var assetIndex = r.RuleValue[0];
                        if (a.Length <= assetIndex)
                        {
                            return false;
                        }

                        return p.IsOwnerOf(a[assetIndex]);
                    }

                default:
                    throw new NotSupportedException($"Unsupported RuleType {r.RuleType}!");
            }
        };
    }

    private static IEnumerable<(GameIdentifier, GameRule[], ITransitioFee?, TransitionFunction<GameRule>)> GetRulesAndTranstionSets()
    {
        var result = new List<(GameIdentifier, GameRule[], ITransitioFee?, TransitionFunction<GameRule>)>
        {
            //EatTransition(),
        };

        return result;
    }

    private static (GameIdentifier, GameRule[], ITransitioFee, TransitionFunction<GameRule>) EatTransition()
    {
        throw new NotImplementedException();
    }
}