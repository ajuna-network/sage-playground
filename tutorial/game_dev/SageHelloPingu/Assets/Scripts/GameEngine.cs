using Ajuna.SAGE.Core;
using Ajuna.SAGE.Core.Manager;
using Ajuna.SAGE.Core.Model;
using SageUnityLib;
using System;
using System.Collections.Generic;
using System.Linq;
using TMPro;
using UnityEngine;
using static UnityEngine.UIElements.UxmlAttributeDescription;

public class GameEngine : MonoBehaviour
{
    public IBlockchainInfoProvider BlockchainInfoProvider { get; private set; }
    
    public Engine<GameIdentifier, GameRule> Engine { get; private set; }

    public IAccount User { get; private set; }

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

        // Create a user account and add some balance to it
        User = Engine.AccountManager.Account(Engine.AccountManager.Create());
        User.Balance.Deposit(1_000_000);
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

                case GameRuleType.AssetTypesAt:
                    {
                        if (r.RuleOpEnum != GameRuleOp.Composite)
                        {
                            return false;
                        }

                        for (int i = 0; i < r.RuleValue.Length; i++)
                        {
                            byte composite = r.RuleValue[i];

                            if (composite == 0)
                            {
                                continue;
                            }

                            byte assetType = composite;

                            if (a.Length <= i)
                            {
                                return false;
                            }

                            var baseAsset = a[i] as BaseAsset;
                            if (baseAsset == null
                            || (byte)baseAsset.AssetType != assetType)
                            {
                                return false;
                            }
                        }

                        return true;
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
            CreatePenguin(),
            CreateFish(),
            EatTransition(),
        };

        return result;
    }

    private static (GameIdentifier, GameRule[], ITransitioFee, TransitionFunction<GameRule>) CreatePenguin()
    {
        var identifier = GameConfig.CreatePenguin(out GameRule[] rules, out ITransitioFee fee);

        TransitionFunction<GameRule> function = (e, r, f, a, h, b, c, m) =>
        {
            var penguin = new PlayerAsset(e.Id, 0, b);

            var result = new IAsset[] { penguin };

            return result;
        };

        return (identifier, rules, fee, function);
    }

    private static (GameIdentifier, GameRule[], ITransitioFee, TransitionFunction<GameRule>) CreateFish()
    {
        var identifier = GameConfig.CreateFish(out GameRule[] rules, out ITransitioFee fee);

        TransitionFunction<GameRule> function = (e, r, f, a, h, b, c, m) =>
        {
            var fish = new ConsumableAsset(e.Id, 0, b);

            var result = new IAsset[] { fish };

            return result;
        };

        return (identifier, rules, fee, function);
    }

    private static (GameIdentifier, GameRule[], ITransitioFee, TransitionFunction<GameRule>) EatTransition()
    {
        var identifier = GameConfig.Eat(out GameRule[] rules, out ITransitioFee fee);

        TransitionFunction<GameRule> function = (e, r, f, a, h, b, c, m) =>
        {
            var penguin = new PlayerAsset(a.ElementAt(0));
            var fish = new ConsumableAsset(a.ElementAt(1));

            var result = new IAsset[] { penguin };

            penguin.Health = (byte)Mathf.Clamp(penguin.Health + fish.HealthValue, 0, 100);

            return result;
        };

        return (identifier, rules, fee, function);
    }
}