# Unity EAT Transition Implementation

This tutorial demonstrates how to implement the **EAT** transition in Unity using SAGE, allowing a Penguin (PlayerAsset) to consume a Fish (ConsumableAsset). It covers rule configuration, transition wiring, and scene integration.

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

- Define and register **EAT** transition identifier and validation rules.
- Implement `EatTransition` function that:
  - Validates asset ownership and types.
  - Increases Penguin’s health by Fish’s `HealthValue`.
  - Returns only the updated Penguin asset (Fish is consumed).
- Hook up a simple Unity UI/button to invoke `ExecuteEat` and observe results.

---

## 1️⃣ Step 1: Update Enums and GameConfig

1. **Enums.cs** - ensure your enums include the following:

   ```csharp
   namespace SageUnityLib
   {
       public enum AssetType : byte
       {
           None = 0,
           Player = 1,
           Consumable = 2,
       }

       public enum GameAction : byte
       {
           None = 0,
           Eat = 1,
       }

       public enum GameRuleType : byte
       {
           None = 0,
           IsOwnerOf = 1,
           AssetTypesAt = 2,
       }

       public enum GameRuleOp : byte
       {
           None = 0,
           Index = 1,
           Composite = 2,
       }
   }
   ```

2. **GameConfig.cs** - add an `Eat` helper to produce identifier, rules, and fee:

   ```csharp
   using Ajuna.SAGE.Core.Model;
   using SageUnityLib;

   public static class GameConfig
   {
       internal static GameIdentifier Eat(out GameRule[] rules, out ITransitionFee fee)
       {
           byte playerAsset = (byte)AssetType.Player;
           byte fishAsset  = (byte)AssetType.Consumable;

           rules = new [] {
               // Validate ownership of both assets
               new GameRule(GameRuleType.IsOwnerOf, GameRuleOp.Index, new byte[]{ 0x00 }),
               new GameRule(GameRuleType.IsOwnerOf, GameRuleOp.Index, new byte[]{ 0x01 }),
               // Ensure types: first Player, then Consumable
               new GameRule(GameRuleType.AssetTypesAt, GameRuleOp.Composite, new byte[]{ playerAsset, fishAsset })
           };

           fee = default; // no fee for this demo

           return new GameIdentifier((byte)GameAction.Eat);
       }
   }
   ```

---

## 2️⃣ Step 2: Implement the Eat Transition Function

1. In **GameEngine.cs**, locate the `EatTransition()` stub and replace with:

   ```csharp
   private static (
       GameIdentifier,
       GameRule[],
       ITransitionFee,
       TransitionFunction<GameRule>)
   EatTransition()
   {
       // Get identifier, rules, fee
       var identifier = GameConfig.Eat(out GameRule[] rules, out ITransitionFee fee);

       // Define the transition function
       TransitionFunction<GameRule> function = (account, ruleSet, assets, balance, payload, balanceMgr, assetMgr) =>
       {
           // Assets array: [0] = Penguin, [1] = Fish
           var penguin = assetMgr.Get<PlayerAsset>(assets[0].Id);
           var fish    = assetMgr.Get<ConsumableAsset>(assets[1].Id);

           // Update health (clamp 0–100)
           penguin.Health = (uint) Mathf.Clamp(
               penguin.Health + fish.HealthValue,
               0, 100
           );

           // Return ONLY the updated Penguin (Fish is consumed)
           return new IAsset[]{ penguin };
       };

       return (identifier, rules, fee, function);
   }
   ```

2. Also in **GameEngine.cs**, ensure that we initially create a user account:
   ```csharp
    public class GameEngine : MonoBehaviour
    {
        ...
        public IAccount User { get; private set; }
        ...
        private void Awake()
        {
            ...
            // Create a user account and add some balance to it
            User = Engine.AccountManager.Account(Engine.AccountManager.Create());
            User.Balance.Deposit(1_000_000);
            ...
        }
   ```
3. Ensure that we now as we have the user that we use the correct ID on our Asset spawners and we allow access to the asset through a getter and setter:
  
   ```csharp
    public class PenguinSpawner : MonoBehaviour
    {
        ...
        public PlayerAsset Penguin { get; set; } // make sure to replace old _player ref., with Penguin
        ...
        [SerializeField]
        private GameEngine _gameEngine;
        ...
        private void Awake()
        {
            ...
            // Create a new Penguin asset
            Penguin = new PlayerAsset(_gameEngine.User.Id);
            ...
        }
   ```

   and same for the **FishSpawner.cs**

   ```csharp
    public class FishSpawner : MonoBehaviour
    {
        ...
        public ConsumableAsset Fish { get; set; } // make sure to replace old _fish ref., with Fish
        ...
        [SerializeField]
        private GameEngine _gameEngine;
        ...
        private void Awake()
        {
            ...
            // Create a new Penguin asset
            Fish = new ConsumableAsset(_gameEngine.User.Id);
            ...
        }
   ```


4. Ensure in add the EatTransition:

   ```csharp
    private static IEnumerable<(GameIdentifier, GameRule[], ITransitioFee?, TransitionFunction<GameRule>)> GetRulesAndTranstionSets()
    {
        var result = new List<(GameIdentifier, GameRule[], ITransitioFee?, TransitionFunction<GameRule>)>
        {
            EatTransition(),
        };

        return result;
    }
   ```

---

> **Note:** Most of this code will be moved into a library so it'snot worked out in best-practice way, in Unity as we will move it in a later step to a proper class library that will expose a strong bestpractice.

## 3️⃣ Step 3: Triggering the Transition from UI

1. Create **TransitionController.cs**:

   ```csharp
   using UnityEngine;
   using SageUnityLib;

   public class TransitionController : MonoBehaviour
   {
       [SerializeField]
       private GameEngine _gameEngine;
    
       [SerializeField]
       private PenguinSpawner _penguin;

       [SerializeField]
       private FishSpawner _fish;

       public void OnEatButton()
       {
           var inputAssets = new IAsset[] { _penguin.Penguin, _fish.Fish };
           var identifier = new GameIdentifier((byte)GameAction.Eat);

           // Execute the EAT transition
           var successFlag = _gameEngine.Engine.Transition(
               _gameEngine.User, // User account
               new GameIdentifier((byte)GameAction.Eat),
               inputAssets,
               out IAsset[] outAssets
           );

           if (!successFlag)
           {
               Debug.LogError("Transition failed: " + (GameAction)identifier.TransitionType);
               return;
           }

           Debug.Log("Transition succeed: " + (GameAction)identifier.TransitionType);

       }
   }
   ```

2. **Scene Setup**:

   - Add a **Button** to the Canvas; in its Inspector, hook **OnClick → EatController.OnEatButton**.
   - Assign references: **GameEngine**, **Penguin** GameObject’s `PlayerAsset` component, and **Fish** GameObject’s `ConsumableAsset`.

3. Pressing the Button should lead to this console output:
   ```
   Transition succeed: Eat
   ```
   If that is done, then let's also make sure the the Penguin's health increases.

   ```csharp
       
       public void OnEatButton()
       {
            ...
            // Handle the output assets
            _penguin.Penguin = outAssets[0] as PlayerAsset;
            Destroy(_fish.gameObject);
       }
   ```

---

## 4️⃣ Step 4: Verify the EAT Action

- **Before** pressing the button: check Penguin’s health (e.g., 10) and Fish exists.
- **After** pressing: console should log new health; health UI updates; Fish GameObject is destroyed.

---

## 🟢 Git Tutorial Checkpoints

| Stage      | Commit Description                            | Git Ref                 |
| ---------- | --------------------------------------------- | ----------------------- |
| 🟢 Start   | Engine foundation + assets in scene, no logic | `replace-this-start`    |
| ✅ Complete | EAT transition implemented & visually tested  | `replace-this-complete` |

Congratulations! You've implemented the **EAT** transition, leveraging SAGE to validate, update asset state, and manage scene objects. Next: extend with animations and integrate backend persistence.

