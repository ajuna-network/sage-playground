using Ajuna.SAGE.Core;
using Ajuna.SAGE.Core.Manager;
using Ajuna.SAGE.Core.Model;
using System;
using System.Collections.Generic;
using System.Linq;
using TMPro;
using UnityEngine;


namespace SageUnityLib
{
  public class GameEngine : MonoBehaviour
  {
    [SerializeField] private TMP_Text _blockNumberTxt;
    public IBlockchainInfoProvider BlockchainInfoProvider { get; private set; }
    public Engine<GameIdentifier, GameRule> Engine { get; private set; }

    public IAccount User { get; private set; }
    
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

      Engine = builder.Build(); // ✅ Engine is ready here

      // Now safe to create the User
      User = Engine.AccountManager.Account(Engine.AccountManager.Create());
      User.Balance.Deposit(1_000_000);
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

          case GameRuleType.AssetTypesAt:
            if (rule.RuleValue == null || rule.RuleValue.Length == 0) return false;
            if (assets.Length != rule.RuleValue.Length) return false;

            for (int i = 0; i < rule.RuleValue.Length; i++)
            {
              if (assets[i] is not BaseAsset assetWithType)
              {
                Debug.LogError($"Asset at index {i} is not a BaseAsset (actual type: {assets[i].GetType()})");
                return false;
              }

              if ((byte)assetWithType.AssetType != rule.RuleValue[i])
              {
                Debug.LogError($"AssetType mismatch at index {i}: expected {rule.RuleValue[i]}, got {(byte)assetWithType.AssetType}");
                return false;
              }
            }

            return true;

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
        EatTransition(),
    };
      return result;
    }

    /// <summary>
    /// Creates a game rule and transition set for eating a fish.
    /// </summary>
    /// <returns></returns>
    private static (GameIdentifier, GameRule[], ITransitioFee?, TransitionFunction<GameRule>) EatTransition()
    {
      var identifier = GameConfig.Eat(out GameRule[] rules, out ITransitioFee fee);
      Debug.Log($"[EatTransition] Identifier: ({identifier.TransitionType}, {identifier.TransitionSubType})");
      TransitionFunction<GameRule> function = (e, r, f, a, h, b, c, m) =>
      {
        var assetsList = a.ToList();

        var penguin = new PenguinAsset((Asset)assetsList[0]);
        var fish = new FishAsset((Asset)assetsList[1]);

        penguin.Health = (byte)Math.Clamp(penguin.Health + fish.HealthValue, 0, 100);

        return new IAsset[] { penguin };
      };

      return (identifier, rules, fee, function);
    }
  }
}