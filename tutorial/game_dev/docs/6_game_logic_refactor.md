# C# Game Logic Refactor for SAGE Integration

This guide extracts core game logic (asset definitions and transition functions) from Unity into a standalone C# Class Library (**SageUnityLib**) to enable full unit testing, cleaner architecture, and Unity-agnostic code.

---

## 📂 Solution Structure

```
YourSolution/
│
├── SageUnityLib/                   # .NET Standard 2.1 Class Library
│   ├── SageUnityLib.csproj
│   ├── Model/
│   │   ├── BaseAsset.cs
│   │   ├── PlayerAsset.cs
│   │   └── ConsumableAsset.cs
│   ├── Enums.cs                    # AssetType, GameAction, GameRuleType, GameRuleOp
│   ├── GameIdentifier.cs           # ITransitionIdentifier struct + statics
│   ├── GameRule.cs                 # ITransitionRule struct
│   ├── GameConfig.cs               # helpers for GameIdentifier and GameRule
│   └── GameEngine.cs               # EngineBuilder wiring and transition funcs
│
├── SageUnityLib.Tests/             # NUnit Test Project
│   ├── SageUnityLib.Tests.csproj
│   ├── BaseSetupTest.cs            # common setup for tests
│   ├── ModelTests/
│   │   ├── PlayerAssetTest.cs
│   │   └── ConsumableAssetTest.cs
│   └── TransitionTests.cs          # tests for Create & DoEat transitions
│
└── YourUnityProject/               # Unity project (unchanged)
    └── ...
```

---

## 🎯 Goal

- Refactor **PlayerAsset**, **ConsumableAsset**, and **BaseAsset** into `SageUnityLib.Model`.
- Centralize enums, `GameIdentifier`, `GameRule`, and `GameConfig` in the library.
- Implement a Unity‑independent `GameEngineService` using `EngineBuilder<TId,TRule>`.
- Add **NUnit** tests covering asset defaults and transition logic (`CreatePenguin`, `CreateFish`, `DoEat`).

---

## 1️⃣ Step 1: Create the Class Library

1. In your solution root:
   ```bash
   dotnet new classlib -n SageUnityLib -f netstandard2.1
   ```
2. Add SAGE Core dependency:
   ```bash
   cd SageUnityLib
   dotnet add package Ajuna.SAGE.Core --version 0.0.6
   ```
3. (Optional) In `SageUnityLib.csproj`, add:
   ```xml
   <PropertyGroup>
     <CopyLocalLockFileAssemblies>true</CopyLocalLockFileAssemblies>
   </PropertyGroup>
   ```

---

## 2️⃣ Step 2: Move Asset Classes

Place these in `SageUnityLib/Model/`:

- **BaseAsset.cs**
- **PenguinAsset.cs**
- **FishAsset.cs**

*(Use `SageUnityLib.Model` namespace and ensure they reference `Ajuna.SAGE.Core.Model`.)*

---

## 3️⃣ Step 3: Enums, Identifier, Rule & Config

In root `SageUnityLib` namespace:

- **Enums.cs**: define `AssetType`, `GameAction {CreatePenguin, CreateFish, Eat}`, `GameRuleType`, `GameRuleOp`.
- **GameIdentifier.cs**:
  ```csharp
  public struct GameIdentifier : ITransitionIdentifier
  {
      ...
      public static GameIdentifier CreatePenguin => new((byte)GameAction.CreatePenguin);
      public static GameIdentifier CreateFish    => new((byte)GameAction.CreateFish);
      public static GameIdentifier Eat         => new((byte)GameAction.Eat);
  }
  ```
- **GameRule.cs**: implement `ITransitionRule` with ctor and `RuleTypeEnum`/`RuleOpEnum` helpers.
- **GameConfig.cs**: static methods returning identifiers and rule arrays:
  - `CreatePenguin`, `CreateFish` (empty rules)
  - `Eat` (IsOwnerOf and AssetTypesAt rules)

---

## 4️⃣ Step 4: Refactor Engine into Service Class

Use a static `GameEngine.Create(...)` method with the following code (no Unity dependencies):

```csharp
using Ajuna.SAGE.Core;
using Ajuna.SAGE.Core.Manager;
using Ajuna.SAGE.Core.Model;
using SageUnityLib;
using System;
using System.Collections.Generic;
using System.Linq;

/// <summary>
/// GameEngine is responsible for creating and managing the game engine instance.
/// </summary>

namespace SageUnityLib.Model
{
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

            var rulesAndTransitions = GetRulesAndTransitionSets();
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
        private static Func<IAccount, GameRule, IAsset[], uint, object?, IBalanceManager, IAssetManager, bool>
            GetVerifyFunction()
        {
            return (p, r, a, b, c, m, s) =>
            {
                switch (r.RuleTypeEnum)
                {
                    case GameRuleType.IsOwnerOf:
                        {
                            if (r.RuleOpEnum != GameRuleOp.Index) return false;
                            if (r.RuleValue == null || r.RuleValue.Length == 0) return false;
                            var assetIndex = r.RuleValue[0];
                            if (a.Length <= assetIndex) return false;
                            return p.IsOwnerOf(a[assetIndex]);
                        }

                    case GameRuleType.AssetTypesAt:
                        {
                            if (r.RuleOpEnum != GameRuleOp.Composite) return false;
                            for (int i = 0; i < r.RuleValue.Length; i++)
                            {
                                byte assetType = r.RuleValue[i];
                                if (assetType == 0) continue;
                                if (a.Length <= i) return false;
                                var baseAsset = a[i] as BaseAsset;
                                if (baseAsset == null || (byte)baseAsset.AssetType != assetType)
                                    return false;
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
        private static IEnumerable<(GameIdentifier, GameRule[], ITransitioFee?, TransitionFunction<GameRule>)>
            GetRulesAndTransitionSets()
        {
            return new List<(GameIdentifier, GameRule[], ITransitioFee?, TransitionFunction<GameRule>)>
        {
            CreatePenguin(),
            CreateFish(),
            EatTransition(),
        };
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
                var penguin = new PenguinAsset(e.Id, null, b);
                return new IAsset[] { penguin };
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
                var fish = new FishAsset(e.Id, null, b);
                return new IAsset[] { fish };
            };
            return (identifier, rules, fee, function);
        }

        /// <summary>
        /// Creates a game rule and transition set for eating a fish.
        /// </summary>
        /// <returns></returns>
        private static (GameIdentifier, GameRule[], ITransitioFee?, TransitionFunction<GameRule>) EatTransition()
        {
            var identifier = GameConfig.Eat(out GameRule[] rules, out ITransitioFee fee);
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
```

---

## 5️⃣ Step 5: Add NUnit Tests

Create a new NUnit test project **SageUnityLib.Tests** in the root folder and include the following test classes:

### BaseSetupTest.cs
```csharp
using Ajuna.SAGE.Core;
using Ajuna.SAGE.Core.Model;
using SageUnityLib.Model;
using NUnit.Framework;

namespace SageUnityLib.Test
{
    public class BaseSetupTest
    {
        public IBlockchainInfoProvider BlockchainInfoProvider { get; private set; }
        public Engine<GameIdentifier, GameRule> Engine { get; private set; }

        [SetUp]
        public void Setup()
        {
            BlockchainInfoProvider = new BlockchainInfoProvider(1234);
            Engine = GameEngine.Create(BlockchainInfoProvider);
        }

        public void Reset()
        {
            BlockchainInfoProvider = new BlockchainInfoProvider(1234);
            Engine = GameEngine.Create(BlockchainInfoProvider);
        }

        public T GetAsset<T>(IAccount user, AssetType type) where T : BaseAsset
        {
            var result = Engine.AssetManager
                .AssetOf(user)
                .OfType<BaseAsset>()
                .FirstOrDefault(p => p.AssetType == type);
            Assert.That(result, Is.Not.Null);
            Assert.That(result, Is.InstanceOf<T>());
            return (T)result;
        }
    }
}
```

### PlayerAssetTest.cs
```csharp
using SageUnityLib.Model;

namespace SageUnityLib.Test.Model
{
    [TestFixture]
    public class PlayerAssetTest
    {
        private PenguinAsset playerAsset;

        [SetUp]
        public void Setup()
        {
            playerAsset = new PenguinAsset(1, null, 0);
        }

        [Test]
        public void Test_PlayerAsset()
        {
            Assert.That(playerAsset, Is.Not.Null);
            Assert.That(playerAsset.AssetType, Is.EqualTo(AssetType.Player));
            Assert.That(playerAsset.Health, Is.EqualTo(95), "Default health should be 95.");
        }
    }
}
```

### ConsumableTest.cs
```csharp
using SageUnityLib.Model;

namespace SageUnityLib.Test.Model
{
    [TestFixture]
    public class ConsumableTest
    {
        private FishAsset consumableAsset;

        [SetUp]
        public void Setup()
        {
            consumableAsset = new FishAsset(1, null, 0);
        }

        [Test]
        public void Test_ConsumableAsset()
        {
            Assert.That(consumableAsset, Is.Not.Null);
            Assert.That(consumableAsset.AssetType, Is.EqualTo(AssetType.Consumable));
            Assert.That(consumableAsset.HealthValue, Is.EqualTo(5), "Default healthValue should be 5.");
        }
    }
}
```

### TransitionTests.cs
```csharp
using Ajuna.SAGE.Core;
using Ajuna.SAGE.Core.Model;
using SageUnityLib.Model;
using NUnit.Framework;

namespace SageUnityLib.Test
{
    public class TransitionTests : BaseSetupTest
    {
        private IAccount _user;

        [SetUp]
        public void Setup()
        {
            var userId = Engine.AccountManager.Create();
            _user = Engine.AccountManager.Account(userId);
            _user.Balance.Deposit(1_000_000);
            BlockchainInfoProvider.CurrentBlockNumber++;
        }

        [Test]
        public void Test_CreatePenguin()
        {
            Assert.That(BlockchainInfoProvider.CurrentBlockNumber, Is.EqualTo(2));
            bool resultFirst = Engine.Transition(_user, GameIdentifier.CreatePenguin, null, out IAsset[] outAssets, null);
            Assert.That(resultFirst, Is.True, "transition result should succeed.");
            var penguin = outAssets[0] as PenguinAsset;
            Assert.That(penguin, Is.Not.Null);
            Assert.That(penguin.Health, Is.EqualTo(95));
        }

        [Test]
        public void Test_CreateFish()
        {
            Assert.That(BlockchainInfoProvider.CurrentBlockNumber, Is.EqualTo(2));
            bool resultFirst = Engine.Transition(_user, GameIdentifier.CreateFish, null, out IAsset[] outAssets, null);
            Assert.That(resultFirst, Is.True, "transition result should succeed.");
            var fish = outAssets[0] as FishAsset;
            Assert.That(fish, Is.Not.Null);
            Assert.That(fish.HealthValue, Is.EqualTo(5));
        }

        [Test]
        public void Test_DoEat()
        {
            Assert.That(BlockchainInfoProvider.CurrentBlockNumber, Is.EqualTo(2));
            Engine.Transition(_user, GameIdentifier.CreatePenguin, null, out _, null);
            Engine.Transition(_user, GameIdentifier.CreateFish, null, out _, null);
            bool result = Engine.Transition(_user, GameIdentifier.Eat, new IAsset[] {
                Engine.AssetManager.AssetOf(_user).First(a => ((BaseAsset)a).AssetType == AssetType.Player),
                Engine.AssetManager.AssetOf(_user).First(a => ((BaseAsset)a).AssetType == AssetType.Consumable)
            }, out IAsset[] outAssets, null);
            Assert.That(result, Is.True, "transition result should succeed.");
            Assert.That(outAssets.Length, Is.EqualTo(1), "should return one asset");
            var updatedPenguin = outAssets[0] as PenguinAsset;
            Assert.That(updatedPenguin, Is.Not.Null, "updated penguin should not be null");
            Assert.That(updatedPenguin.Health, Is.EqualTo(100), "penguin health should be 100 after eating fish");
        }
    }
}
```

---

Make sure all tests pass with:
```bash
cd SageUnityLib.Tests
dotnet test
```

---

## 📚 Documentation

- **File**: `/docs/csharp/game_logic_refactor.md` (this tutorial)
- Include project creation commands, folder layout, key code excerpts, and sample test outputs.
- Explain how to build and copy `SageUnityLib.dll` into Unity’s `Assets/Plugins/`.

---

## 🟢 Git Tutorial Checkpoints

| Stage      | Commit Description                                              | Git Ref              |
| ---------- | --------------------------------------------------------------- | -------------------- |
| 🟢 Start   | EAT logic embedded in Unity project                             | [2f4fbae277b289ddc8ff670d398dedbdc065c4f9](https://github.com/ajuna-network/sage-playground/commit/2f4fbae277b289ddc8ff670d398dedbdc065c4f9) |
| ✅ Complete| Logic extracted into `SageUnityLib` with NUnit tests running     | [996e9af75c62667e4578c2456daf4ec50adc2ad1](https://github.com/ajuna-network/sage-playground/commit/996e9af75c62667e4578c2456daf4ec50adc2ad1) |


Congratulations! You now have a fully decoupled, testable game logic library ready for integration into Unity or other game engines.

