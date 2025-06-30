# Unity Game Engine Setup for SAGE Integration

This guide prepares the core engine foundation in Unity to support SAGE-based transitions (e.g., `EAT`). It defines common engine setup and transition wiring for future tutorials.

---

## 📂 Folder Structure

```text
YourUnityProject/
│
└── Assets/
    ├── Scripts/
    │   ├── GameEngine/            # Core game logic library
    │   │   ├── BaseAsset.cs
    │   │   ├── Enums.cs           # Action and rule enums
    │   │   ├── GameIdentifier.cs  # Transition identifier
    │   │   ├── GameRule.cs        # Transition rule struct
    │   │   ├── PlayerAsset.cs     # Player (Penguin) asset
    │   │   └── ConsumableAsset.cs # Consumable (Fish) asset
    │   ├── PenguinSpawner.cs      # Spawns and displays Penguin
    │   ├── FishSpawner.cs         # Spawns and displays Fish
    │   └── GameEngine.cs          # The game engine MonoBehaviour
    ├── Plugins/                  # Ajuna.SAGE.Core DLLs
    └── Scenes/                   # Unity scenes
        └── MainScene.unity      # Example scene
```

---

## 🎯 Goal

- Establish a reusable **GameEngine** MonoBehaviour to initialize the SAGE engine.
- Define and register transition identifiers and rules for interactions (e.g., `Eat`).
- Wire up asset classes (`PlayerAsset`, `ConsumableAsset`) and spawners.

---

## 1️⃣ Step 1: Define Transition Identifiers and Rules

We'll define the available actions, how to identify each transition, and validation rules.

1. **Action Enum** (`GameAction`): Add to **Enums.cs**:

   ```csharp
   namespace SageUnityLib
   {

       // Add this block below the `AssetType` Enum
       public enum GameAction : byte
       {
           None = 0,
           Eat = 1,
       }
   }
   ```

2. **Transition Identifier** (`GameIdentifier`): In **Assets/Scripts/GameEngine**, create **GameIdentifier.cs** :

   ```csharp
   using Ajuna.SAGE.Core.Model;

   public class GameIdentifier : ITransitionIdentifier
   {
       public byte TransitionType { get; set; }
       public byte TransitionSubType { get; set; }

       public GameIdentifier(byte transitionType, byte transitionSubType)
       {
           TransitionType = transitionType;
           TransitionSubType = transitionSubType;
       }

       public GameIdentifier(byte transitionType)
           : this(transitionType, 0)
       {
       }
   }
   ```

3. Add **Rule Enums** (`GameRuleType`, `GameRuleOp`): Add to **Enums.cs**:

   ```csharp
   namespace SageUnityLib
   {

       // Add these blocks below the `GameAction` Enums:
       public enum GameRuleType : byte
       {
           None = 0,
           IsOwnerOf = 1,
       }

       public enum GameRuleOp : byte
       {
           None = 0,
           Index = 1,
       }
   }
   ```

4. **Transition Rule** (`GameRule`): In **Assets/Scripts/GameEngine**, create **GameRule.cs**:

   ```csharp
   using Ajuna.SAGE.Core.Model;
   using System;

   namespace SageUnityLib
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
   ```

---

## 2️⃣ Step 2: Initialize the GameEngine

1. Create **GameEngine.cs** in **Assets/Scripts/GameEngine**:

    ```csharp
    using Ajuna.SAGE.Core;
    using Ajuna.SAGE.Core.Manager;
    using Ajuna.SAGE.Core.Model;
    using System;
    using System.Collections.Generic;
    using UnityEngine;

    namespace SageUnityLib
    {
        public class GameEngine : MonoBehaviour
        {
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
    ```

2. In the Scene: Create an empty GameObject named `GameEngine` and attach the `GameEngine` component.

---

## 3️⃣ (Optional) Step 3: Display Blocknumber in UI

1. **UI Setup**:
   - In **Hierarchy**, right‑click → **UI → Canvas**.
   - Under the Canvas, right‑click → **UI → Text** (or **TextMeshPro**).
   - Name it `BlockNumberText` and position it.
2. **Update GameEngine.cs** in `Assets/Scripts/`:

```csharp
    // Add this line to your imports at the top of the file
    using TMPro;
...
    // Add these lines at the top of the GameEngine class:
    [SerializeField]
    private TMP_Text _blockNumberTxt;
...
    // Replace the existing UpdateBlockNumber function with this code block
    private void UpdateBlockNumber()
    {
        BlockchainInfoProvider.CurrentBlockNumber++;
        Debug.Log($"Blocknumber: {BlockchainInfoProvider.CurrentBlockNumber}");
        if (_blockNumberTxt != null)   
        {
            _blockNumberTxt.text = $"BLOCK: {BlockchainInfoProvider.CurrentBlockNumber}";
        }
    }
...
```

3. Drag the **BlockNumberText** GameObject to the `GameEngine` `Block Number Text` inspector slot.
4. Press **Play**. You should see **BLOCK: 1** for the actual blocknumber (or your chosen start value). The value will incre

![Unity Console](https://github.com/eca20/sage-playground/blob/battle-test/tutorial/game_dev/docs/images/Screenshot%202025-06-27%20at%205.00.33%E2%80%AFPM.png?raw=true)

---

## 🟢 Git Tutorial Checkpoints

| Stage      | Commit Description                | Git Ref                 |
| ---------- | --------------------------------- | ----------------------- |
| 🟢 Start   | Fish asset created and placed in scene | [d174b5103b270f8434bb06583bd4f4824f829aef](https://github.com/ajuna-network/sage-playground/commit/d174b5103b270f8434bb06583bd4f4824f829aef)    |
| ✅ Complete | Core engine implemented and wired | [fb92aa199b990fde3fa67f45959f79e85184a868](https://github.com/ajuna-network/sage-playground/commit/fb92aa199b990fde3fa67f45959f79e85184a868) |

Congratulations! You now have a solid engine setup to power asset registration and transition execution. Next: implement the **EAT** transition logic and integrate it into your scene.

