# Unity CREATE Transition Implementation

This tutorial shows how to enforce asset instantiation through SAGE `CREATE` transitions for both the Penguin (PlayerAsset) and Fish (ConsumableAsset), ensuring on‑chain semantics and consistent block/genesis metadata.

---

## 📂 Folder Structure

```
YourUnityProject/
│
└── Assets/
    ├── Scripts/
    │   ├── GameEngine/            # Core engine code
    │   │   ├── BaseAsset.cs       # Base Asset
    │   │   └── ConsumableAsset.cs # Consumable (Fish) asset
    │   │   ├── Enums.cs           # AssetType, GameAction, GameRuleType, GameRuleOp
    │   │   ├── GameIdentifier.cs  # ITransitionIdentifier impl
    │   │   ├── GameRule.cs        # ITransitionRule impl
    │   │   ├── PlayerAsset.cs     # Player (Penguin) asset
    │   ├── FishSpawner.cs         # Spawns Fish
    │   ├── GameConfig.cs          # Configuration for Eat transition
    │   ├── GameEngine.cs          # GameEngine MonoBehaviour
    │   ├── PenguinSpawner.cs      # Spawns Penguin
    │   └── EatController.cs       # UI/button to trigger ExecuteEat
    ├── Plugins/                  # Ajuna.SAGE.Core DLLs
    └── Scenes/                   # Unity scenes
        └── MainScene.unity      # Contains Penguin, Fish, GameEngine
```

---

## 🎯 Goal

- Add `CREATE` transitions for Penguin and Fish assets in SAGE.
- Update the engine registration to include `CreatePenguin`, `CreateFish`, and existing `Eat` transitions.
- Replace manual spawner instantiation with calls to `Engine.Transition()`.
- Ensure each asset’s `ownerId` and `genesis` (block number) are set by the engine.

---

## 1️⃣ Step 1: Update `GameAction` Enum

1. In **GameEngine/Enums.cs**, modify `GameAction` to include creation actions:

    ```csharp
    namespace SageUnityLib
    {
        public enum GameAction : byte
        {
            None           = 0,
            CreatePenguin  = 1,
            CreateFish     = 2,
            Eat            = 3,  // moved from 1 to 3
        }
    }
    ```

---

## 2️⃣ Step 2: Add CREATE Identifiers to `GameConfig`

 1. In **GameEngine/GameConfig.cs**, add two static helpers:

    ```csharp
    using Ajuna.SAGE.Core.Model;
    using SageUnityLib;

    public static class GameConfig
    {
        internal static GameIdentifier CreatePenguin(out GameRule[] rules, out ITransitionFee fee)
        {
            rules = new GameRule[] { };
            fee   = default;
            return new GameIdentifier((byte)GameAction.CreatePenguin);
        }

        internal static GameIdentifier CreateFish(out GameRule[] rules, out ITransitionFee fee)
        {
            rules = new GameRule[] { };
            fee   = default;
            return new GameIdentifier((byte)GameAction.CreateFish);
        }

        // Existing Eat helper remains here...
    }
    ```

---

## 3️⃣ Step 3: Implement CREATE Transitions in `GameEngine`

 1. In **GameEngine/GameEngine.cs**, define two new transition functions alongside `EatTransition()`:

    ```csharp
    private static (
          GameIdentifier,
          GameRule[],
          ITransitioFee,
          TransitionFunction<GameRule>)
          CreatePenguin()
        {
          var id = GameConfig.CreatePenguin(out GameRule[] rules, out ITransitioFee fee);
          TransitionFunction<GameRule> fn = (account, ruleSet, assets, balance, payload, bm, am, mm) =>
          {
            // 'account' holds creator info, 'bm' as genesis
            var penguin = new PenguinAsset(
              account.Id,                 // ownerId from IAccount
              initialHealth: 10,           // default health
              genesis: bm
              );
            return new IAsset[] { penguin };
          };
          return (id, rules, fee, fn);
        }

        private static (
          GameIdentifier,
          GameRule[],
          ITransitioFee,
          TransitionFunction<GameRule>)
          CreateFish()
        {
          var id = GameConfig.CreateFish(out GameRule[] rules, out ITransitioFee fee);
          TransitionFunction<GameRule> fn = (account, ruleSet, assets, balance, payload, bm, am, mm) =>
          {
            var fish = new FishAsset(
              account.Id,
              healthValue: 5,
              genesis: bm
              );
            return new IAsset[] { fish };
          };
          return (id, rules, fee, fn);
        }
    ```

 2. Then update your registration in `GetRulesAndTransitionSets()`:

    ```csharp
        var result = new List<(GameIdentifier, GameRule[], ITransitioFee?, TransitionFunction<GameRule>)>
        {
            CreatePenguin(),
            CreateFish(),
            EatTransition(),
        };
    ```

*(Or if using **`foreach`**, include these calls before **`EatTransition()`**.)*

---

## 4️⃣ Step 4: Refactor Assets to for use with CREATE

 1. Update the PenguinAsset constructor in **PenguinAsset.cs** and add GenesisBlock:

    ```csharp
        public PenguinAsset(uint ownerId, uint? initialHealth = null, uint genesis = 0)
            : base(ownerId)
        {
          AssetType = AssetType.Player;
          Health = (byte)(initialHealth ?? 10); // If null, use 10
          GenesisBlock = genesis;
        }

        public PenguinAsset(Asset existingAsset) : base(existingAsset.OwnerId)
        {
          AssetType = AssetType.Player;
          Health = existingAsset.Data.Read<byte>(1);
          GenesisBlock = existingAsset.Data.Read<uint>(2);
        }

        /// <summary>
        /// Genesis block number stored at index 2.
        /// </summary>
        public uint GenesisBlock
        {
          get => Data.Read<uint>(2);
          set => Data.Set<uint>(2, value);
        }

    ```

 2. Update the FishAsset constructor in **FishAsset.cs** and add GenesisBlock:

    ```csharp
      public FishAsset(uint ownerId, uint? healthValue = null, uint genesis = 0)
          : base(ownerId)
        {
          AssetType = AssetType.Consumable;
          HealthValue = (byte)(healthValue ?? 5);
          GenesisBlock = genesis;
        }

        public FishAsset(Asset existingAsset) : base(existingAsset.OwnerId)
        {
          AssetType = AssetType.Consumable;
          HealthValue = existingAsset.Data.Read<byte>(1);
          GenesisBlock = existingAsset.Data.Read<uint>(2);
        }

        /// <summary>
        /// Genesis block number stored at index 2.
        /// </summary>
        public uint GenesisBlock
        {
          get => Data.Read<uint>(2);
          set => Data.Set<uint>(2, value);
        }
    ```

---

## 5️⃣ Step 5: Refactor Spawners to Use CREATE

### PenguinSpawner.cs

Replace manual `new PenguinAsset(...)` with:

```csharp
// Add import for Ajuna.SAGE.Core.Model
using Ajuna.SAGE.Core.Model;

// Replace Awake() method with this:
void Start()
{
    // Execute CREATE transition
    bool ok = _gameEngine.Engine.Transition(
        _gameEngine.User,
        new GameIdentifier((byte)GameAction.CreatePenguin),
        null,
        out IAsset[] outAssets
    );
    if (!ok) { Debug.LogError("Failed to create Penguin"); return; }

    Penguin = outAssets[0] as PlayerAsset;
    Debug.Log($"Created Penguin (Health: {Penguin.Health}, Genesis: {Penguin.Genesis})");
}
```

### FishSpawner.cs

Similarly:

```csharp
// Add import for Ajuna.SAGE.Core.Model
using Ajuna.SAGE.Core.Model;

// Replace Awake() method with this:
void Start()
{
    bool ok = _gameEngine.Engine.Transition(
        _gameEngine.User,
        new GameIdentifier((byte)GameAction.CreateFish),
        null,
        out IAsset[] outAssets
    );
    if (!ok) { Debug.LogError("Failed to create Fish"); return; }

    Fish = outAssets[0] as ConsumableAsset;
    Debug.Log($"Created Fish (Value: {Fish.HealthValue}, Genesis: {Fish.Genesis})");
}
```

Now all asset instantiation flows through the SAGE engine, guaranteeing consistent `ownerId` and `genesis` assignment.

---

## 🟢 Git Tutorial Checkpoints

| Stage      | Commit Description                                  | Git Ref                 |
| ---------- | --------------------------------------------------- | ----------------------- |
| 🟢 Start   | Spawners manually `new`-ing assets                  | [423a965856938e58e4584c2878c1213f3b168b31](https://github.com/ajuna-network/sage-playground/commit/423a965856938e58e4584c2878c1213f3b168b31)    |
| ✅ Complete | CREATE transitions implemented; spawners use engine | [1d819b535ce5e2d4272db14b468a669758851e4c](https://github.com/ajuna-network/sage-playground/commit/1d819b535ce5e2d4272db14b468a669758851e4c) |

Congratulations! You’ve enforced proper asset creation via SAGE `CREATE` transitions. Next: integrate persistent backend or mock as needed for demo/testing.
