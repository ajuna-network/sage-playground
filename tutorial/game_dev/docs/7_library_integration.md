# Unity Integration of `SageUnityLib` DLL

This guide shows how to integrate the standalone **SageUnityLib** C# library back into your Unity project, replacing in‑scene logic with calls to the shared, testable module.

---

## 📂 Folder Structure (After Integration)

```
YourUnityProject/
│
└── Assets/
    ├── Plugins/                    # Third‑party DLLs
    │   ├── Ajuna.SAGE.Core.dll
    │   ├── Ajuna.SAGE.Generic.dll
    │   └── SageUnityLib.dll        # Newly built game logic library
    └── Scripts/
        ├── GameEngineService.cs    # Renamed entrypoint (uses SageUnityLib)
        ├── PenguinSpawner.cs       # Updated to use SageUnityLib.Model
        ├── FishSpawner.cs          # Updated to use SageUnityLib.Model
        └── TransitionController.cs # (unchanged or adjusted to GameAction.DoEat)
```

---

## 🎯 Goal

- Build and import **SageUnityLib.dll** into Unity.
- Remove redundant in‑Unity logic (`GameEngine` folder and scripts).
- Rename and simplify **GameEngineService.cs** to interact with `SageUnityLib.GameEngine`.
- Update spawners to reference library types and use `GameAction.DoEat`.

---

## 1️⃣ Step 1: Build the Class Library

1. In your solution, switch to the **SageUnityLib** folder.
2. Run a Release build:
   ```bash
   dotnet build -c Release
   ```
3. Locate the compiled DLL at: (Debug or Release)
   ```
   SageUnityLib/bin/Debug/netstandard2.1/SageUnityLib.dll
   ```

---

## 2️⃣ Step 2: Import DLL into Unity

1. In **UnityDemoProject**, open **Assets/Plugins/** (create if missing).
2. Copy **SageUnityLib.dll** into **Assets/Plugins/**.
3. Remove the entire **Assets/Scripts/GameEngine/** folder—it’s now in the library.

---

## 3️⃣ Step 3: Update GameEngineService Script

1. Rename your existing **GameEngine.cs** to **GameEngineService.cs**.
2. Replace its contents with:
   ```csharp
    using Ajuna.SAGE.Core;
    using Ajuna.SAGE.Core.Model;
    using SageUnityLib;
    using TMPro;
    using UnityEngine;
    using SageUnityLib.Model;

    public class GameEngineService : MonoBehaviour
    {
        public IBlockchainInfoProvider BlockchainInfoProvider { get; private set; }
    
        public Engine<GameIdentifier, GameRule> Engine { get; private set; }

        public IAccount User { get; private set; }

        [SerializeField]
        private TMP_Text _blockNumberTxt;

        private void Awake()
        {
            BlockchainInfoProvider = new BlockchainInfoProvider(1234);
            Engine = GameEngine.Create(BlockchainInfoProvider);

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
    }
   ```

---

## 4️⃣ Step 4: Update Spawner Scripts

### PenguinSpawner.cs

1. Add `using SageUnityLib.Model;` at the top.
2. Inject the engine service:
   ```csharp
   [SerializeField]
   private GameEngineService _gameEngine;
   ```

   Complete Code:
   ```csharp
    using Ajuna.SAGE.Core.Model;
    using UnityEngine;
    using UnityEngine.UI;
    using TMPro;
    using SageUnityLib;
    using SageUnityLib.Model;

    public class PenguinSpawner : MonoBehaviour
    {
      public PenguinAsset Penguin { get; set; }

      [SerializeField]
      private Image _penguinAvatarImg;

      [SerializeField]
      private TMP_Text _penguinHealthTxt;

      [SerializeField]
      private GameEngineService _gameEngine;  
      void Awake()
      {
        // Create a new Penguin asset
        // Penguin = new PenguinAsset(_gameEngine.User.Id); // we use for now ownerId = 1
        // Debug.Log($"Spawned Penguin with Health: {Penguin.Health}");
      }
      
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

        Penguin = outAssets[0] as PenguinAsset;
        Debug.Log($"Created Penguin (Health: {Penguin.Health}, Genesis: {Penguin.Genesis})");
      }

      void Update()
      {
        // Update the health text in the UI
        if (Penguin != null && _penguinAvatarImg != null && _penguinHealthTxt != null)
        {
          // Update the penguin's health
          _penguinAvatarImg.fillAmount = Penguin.Health / 100f; // Assuming Health is between 0 and 100
          _penguinHealthTxt.text = Penguin.Health.ToString();
        }
      }
    }
   ```

### FishSpawner.cs

- Mirror the above changes.

### TransitionController.cs (if applicable)

- Update TransitionController to use **GameEngineService** and Destroy game objects that are Consumed

    ```csharp
    using Ajuna.SAGE.Core.Model;
    using UnityEngine;
    using SageUnityLib;
    using SageUnityLib.Model;

    public class TransitionController : MonoBehaviour
    {
      [SerializeField]
      private GameEngineService _gameEngine;

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
        else
        {
          _penguin.Penguin = outAssets[0] as PenguinAsset;
          Destroy(_fish._fishAvatarImg.gameObject);
          Destroy(_fish._fishHealthTxt.gameObject);
          Destroy(_fish.gameObject); // optional: only if you want to remove the spawner
        }

        Debug.Log("Transition succeed: " + (GameAction)identifier.TransitionType);

      }
    }
    ```

---

## 5️⃣ Step 5: Verify Integration

1. **Compile**: Unity should now compile without errors.
2. **Run**: Press Play:
   - Block number updates appear in Console and on-screen.
   - `Create` spawners instantiate assets via SAGE engine.
   - `Eat` button continues to work using `DoEat`.

![Unity Console](https://github.com/eca20/sage-playground/blob/battle-test/tutorial/game_dev/docs/images/Screenshot%202025-06-28%20at%203.40.57%E2%80%AFPM.png?raw=true)

---

## 🟢 Git Tutorial Checkpoints

| Stage      | Commit Description                                        | Git Ref                 |
| ---------- | --------------------------------------------------------- | ----------------------- |
| 🟢 Start   | Logic extracted into `SageUnityLib` with NUnit tests running | [e831f70a396659ee9c5de4ee085d762fae4d997e](https://github.com/ajuna-network/sage-playground/commit/e831f70a396659ee9c5de4ee085d762fae4d997e) |
| ✅ Complete | Logic moved to SageUnityLib.dll, Unity scripts updated    | [5e0edbd75eb6930b6fd4e259743a381b565193de](https://github.com/ajuna-network/sage-playground/commit/bd00ec0cc475fafaffcff87e7139daa23c26515e) |

Congratulations! Your Unity project now leverages the external **SageUnityLib** for all game logic, keeping the scene clean and testable.