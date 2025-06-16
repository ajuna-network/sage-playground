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

### FishSpawner.cs

- Mirror the above changes.

### TransitionController.cs (if applicable)

- Ensure any calls to `GameAction.Eat` now use `GameIdentifier.DoEat`.

---

## 5️⃣ Step 5: Verify Integration

1. **Compile**: Unity should now compile without errors.
2. **Run**: Press Play:
   - Block number updates appear in Console and on-screen.
   - `Create` spawners instantiate assets via SAGE engine.
   - `Eat` button continues to work using `DoEat`.

---

## 🟢 Git Tutorial Checkpoints

| Stage      | Commit Description                                        | Git Ref                 |
| ---------- | --------------------------------------------------------- | ----------------------- |
| 🟢 Start   | Game logic still inside Unity (GameEngine folder present) | `replace-this-start`    |
| ✅ Complete | Logic moved to SageUnityLib.dll, Unity scripts updated    | `replace-this-complete` |

Congratulations! Your Unity project now leverages the external **SageUnityLib** for all game logic, keeping the scene clean and testable.

