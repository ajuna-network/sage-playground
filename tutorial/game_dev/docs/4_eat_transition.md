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
           // Add this line
           AssetTypesAt = 2,
       }

       public enum GameRuleOp : byte
       {
           None = 0,
           Index = 1,
           // Add this line
           Composite = 2,
       }
   }
   ```

2. Create **GameConfig.cs** under the **Assets/Scripts** directory - add an `Eat` helper to produce identifier, rules, and fee:

   ```csharp
    using Ajuna.SAGE.Core.Model;
    using SageUnityLib;

    public static class GameConfig
    {
        internal static GameIdentifier Eat(out GameRule[] rules, out ITransitioFee fee)
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

## 2️⃣ Step 2: Additional Constructor's and Eat Transition Function

1. Add these constructores to the **PenguinAsset.cs** and **FishAsset.cs** classes

    ```csharp
    // Add this constructor to PenguinAsset.cs
    public PenguinAsset(Asset existingAsset) : base(existingAsset.OwnerId)
    {
      AssetType = AssetType.Player;
      Health = existingAsset.Data.Read<byte>(1);
    }
    ```

    ```csharp
    // Add this constructor to FishAsset.cs
    public FishAsset(Asset existingAsset) : base(existingAsset.OwnerId)
    {
      AssetType = AssetType.Player;
      Health = existingAsset.Data.Read<byte>(1);
    }
    ```

2. Update **PenguinSpawner.cs** and **FishSpawner.cs** access:

   ```csharp
    public PenguinAsset Penguin { get; set; }
   ```

   ```csharp
    public FishAsset Fish { get; set; }

    [SerializeField]
    public Image _fishAvatarImg;

    [SerializeField]
    public TMP_Text _fishHealthTxt;
     ```

3. In **GameEngine.cs**, locate the `EatTransition()` stub and replace with:

   ```csharp

    // Add import for `System.Linq`
    using System.Linq;

    // Add implementation for EatTransition()
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
    ```

4. Also in **GameEngine.cs**, ensure that we initially create a user account:

   ```csharp
    public class GameEngine : MonoBehaviour
    {
        ...
        public IAccount User { get; private set; }
        ...
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
   ```

5. As we have the `User`, ensure that we use the correct ID on our Asset spawners and we allow access to the asset through a getter and setter:
  
   ```csharp
    public class PenguinSpawner : MonoBehaviour
    {
        ...
        public PlayerAsset Penguin { get; set; } // make sure to replace old _penguin ref., with Penguin
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

6. Ensure this is added along with the EatTransition:

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
    using Ajuna.SAGE.Core.Model;
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
        var identifier = GameConfig.Eat(out _, out _);
        Debug.Log($"[OnEatButton] Trying transition: ({identifier.TransitionType}, {identifier.TransitionSubType})");
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

   - Add a **Button** to the Canvas; in its Inspector, hook **OnClick → TransitionController.OnEatButton** buy dragging the TransitionController into the **OnClick** section in the inspector and hooking the **OnEatButton** function. 
   - Assign references: **GameEngine**, **Penguin** GameObject’s `PlayerAsset` component, and **Fish** GameObject’s `ConsumableAsset`.

![Unity Console](https://github.com/ajuna-network/sage-playground/blob/tutorial/tutorial/game_dev/docs/images/Screenshot%202025-06-16%20135253.png?raw=true)

3. Pressing the Button should lead to this console output:

   ```
   Transition succeed: Eat
   ```

   If that is done, then let's also make sure the the Penguin's health increases and that the Sardine is removed from the screen.

   ```csharp
    if (!successFlag)
    {
      Debug.LogError("Transition failed: " + (GameAction)identifier.TransitionType);
      return;
    }
    else
    {
      _penguin.Penguin = outAssets[0] as PenguinAsset;
      Destroy(_fish._fishAvatarImg.gameObject);
      Destroy(_fish._fishHealthTxt.gameObject);
      Destroy(_fish.gameObject); // optional: only if you want to remove the spawner
    }
   ```

   Also in **PenguinAsset.cs** ensure the constructor sets health from value passed in signature:

   ```csharp
       public PenguinAsset(uint ownerId, uint initialHealth = 10)
        : base(ownerId)
    {
      AssetType = AssetType.Player;
      Health = (byte)initialHealth;
    }
    ```

![Unity Console](https://github.com/ajuna-network/sage-playground/blob/tutorial/tutorial/game_dev/docs/images/Screenshot%202025-06-16%20135405.png?raw=true)

---

## 4️⃣ Step 4: Verify the EAT Action

- **Before** pressing the button: check Penguin’s health (e.g., 10) and Fish exists.
- **After** pressing: console should log new health; health UI updates; Fish GameObject is destroyed.

---

## 🟢 Git Tutorial Checkpoints

| Stage      | Commit Description                            | Git Ref                 |
| ---------- | --------------------------------------------- | ----------------------- |
| 🟢 Start   | Engine foundation + assets in scene, no logic | [ba618197652c2bdcf350c371c634b0cca1ac564a](https://github.com/ajuna-network/sage-playground/commit/ba618197652c2bdcf350c371c634b0cca1ac564a) |
| ✅ Complete | EAT transition implemented & visually tested  | [71f77320f70805d9ceaa8c0f22bde727b51e73de](https://github.com/ajuna-network/sage-playground/commit/71f77320f70805d9ceaa8c0f22bde727b51e73de) |

Congratulations! You've implemented the **EAT** transition, leveraging SAGE to validate, update asset state, and manage scene objects. Next: extend with animations and integrate backend persistence.
