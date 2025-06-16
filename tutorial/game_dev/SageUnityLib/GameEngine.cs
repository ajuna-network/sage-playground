using Ajuna.SAGE.Core;
using Ajuna.SAGE.Core.Manager;
using Ajuna.SAGE.Core.Model;
using SageUnityLib.Model;
using System;
using System.Collections.Generic;
using System.Linq;

namespace SageUnityLib
{
    /// <summary>
    /// GameEngine is responsible for creating and managing the game engine instance.
    /// </summary>
    public class GameEngine
    {
        /// <summary>
        /// Creates a new game engine instance with the specified blockchain info provider.
        /// </summary>
        /// <param name="blockchainInfoProvider"></param>
        /// <returns></returns>
        public static Engine<GameIdentifier, GameRule> Create(IBlockchainInfoProvider blockchainInfoProvider)
        {
            var engineBuilder = new EngineBuilder<GameIdentifier, GameRule>(blockchainInfoProvider);

            _ = engineBuilder.SetVerifyFunction(GetVerifyFunction());

            var rulesAndTransitions = GetRulesAndTranstionSets();
            foreach (var (identifier, rules, fee, transition) in rulesAndTransitions)
            {
                engineBuilder.AddTransition(identifier, rules, fee, transition);
            }

            return engineBuilder.Build();
        }

        /// <summary>
        /// Returns a function that verifies if the game rules are satisfied for the given account, game rule, assets, block number, context, balance manager, and asset manager.
        /// </summary>
        /// <returns></returns>
        /// <exception cref="NotSupportedException"></exception>
        private static Func<IAccount, GameRule, IAsset[], uint, object?, IBalanceManager, IAssetManager, bool> GetVerifyFunction()
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

        /// <summary>
        /// Returns a collection of game rules and transition sets for the game engine.
        /// </summary>
        /// <returns></returns>
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

        /// <summary>
        /// Creates a penguin game rule and transition set.
        /// </summary>
        /// <returns></returns>
        private static (GameIdentifier, GameRule[], ITransitioFee?, TransitionFunction<GameRule>) CreatePenguin()
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

        /// <summary>
        /// Creates a fish game rule and transition set.
        /// </summary>
        /// <returns></returns>
        private static (GameIdentifier, GameRule[], ITransitioFee?, TransitionFunction<GameRule>) CreateFish()
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

        /// <summary>
        /// Creates a game rule and transition set for eating a fish.
        /// </summary>
        /// <returns></returns>
        private static (GameIdentifier, GameRule[], ITransitioFee?, TransitionFunction<GameRule>) EatTransition()
        {
            var identifier = GameConfig.DoEat(out GameRule[] rules, out ITransitioFee fee);

            TransitionFunction<GameRule> function = (e, r, f, a, h, b, c, m) =>
            {
                var penguin = new PlayerAsset(a.ElementAt(0));
                var fish = new ConsumableAsset(a.ElementAt(1));

                var result = new IAsset[] { penguin };

                penguin.Health = (byte)Math.Clamp(penguin.Health + fish.HealthValue, 0, 100);

                return result;
            };

            return (identifier, rules, fee, function);
        }
    }
}