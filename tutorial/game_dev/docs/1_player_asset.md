# Unity Player Asset (Penguin) Setup

This guide shows you how to define and display a **Player** asset (Penguin) in Unity using the `Ajuna.SAGE.Core` reference asset structure.

---

## 📂 Folder Structure

```
YourUnityProject/
│
└── Assets/
    ├── Scripts/
    │   ├── GameEngine/            # Core game logic library
    │   │   ├── BaseAsset.cs
    │   │   ├── Enums.cs
    │   │   └── PlayerAsset.cs     # Specific Player asset
    │   └── HelloSage.cs           # (From setup tutorial)
    ├── Plugins/                   # Ajuna.SAGE.Core DLLs
    └── Scenes/
        └── MainScene.unity
```

---

## 🎯 Goal

1. Create a **BaseAsset** class that extends `Ajuna.SAGE.Core.Model.Asset`.
2. Define an **AssetType** enum and store it in the asset’s data.
3. Implement a **PlayerAsset** class inheriting **BaseAsset**, with a `Health` property.
4. Instantiate the Penguin in the scene and display its current health via Unity UI.

---

## 1️⃣ Step 1: Create the GameEngine Library Folder

1. In Unity’s **Project** window, right‑click `Assets/Scripts/` → **Create → Folder** → name it **GameEngine**.
2. All core asset definitions will live here (you can later migrate this to a separate .NET class library).

---

## 2️⃣ Step 2: Define BaseAsset

1. In **Assets/Scripts/GameEngine/**, create **BaseAsset.cs**.
2. Add the following code:

```csharp
using Ajuna.SAGE.Core;
using Ajuna.SAGE.Core.Model;

namespace SageUnityLib
{
    /// <summary>
    /// Base class for all SAGE assets in this game.
    /// </summary>
    public class BaseAsset : Asset
    {
        /// <summary>
        /// Constructs an asset with a random ID and 32‑byte data payload.
        /// </summary>
        public BaseAsset(uint ownerId, uint score = 0, uint genesis = 0)
            : base(
                Utils.GenerateRandomId(), // random unique ID
                ownerId,
                collectionId: 1,
                score,
                genesis,
                data: new byte[32]
            )
        {
        }
    }
}
```

> **Note:** `Utils.GenerateRandomId()` is provided by SAGE to create unique IDs.

![Unity Console](https://github.com/ajuna-network/sage-playground/blob/tutorial/tutorial/game_dev/docs/images/Screenshot%202025-06-12%20094514.png?raw=true)

---

## 3️⃣ Step 3: Define AssetType Enum

1. In the same folder, create **Enums.cs**.
2. Define asset categories:

```csharp
namespace SageUnityLib
{
    public enum AssetType : byte
    {
        None = 0,
        Player = 1,
        Consumable = 2,
    }
}
```

---

## 4️⃣ Step 4: Store AssetType in BaseAsset Data

Update **BaseAsset.cs** to read/write the asset type in the first byte of `Data`:

```csharp
using Ajuna.SAGE.Core;
using Ajuna.SAGE.Core.Model;

namespace SageUnityLib
{
    public class BaseAsset : Asset
    {
        public BaseAsset(uint ownerId, uint score = 0, uint genesis = 0)
            : base(Utils.GenerateRandomId(), ownerId, 1, score, genesis, new byte[32])
        {
        }

        /// <summary>
        /// Identifies which kind of asset this is.
        /// Stored in Data[0].
        /// </summary>
        public AssetType AssetType
        {
            get => (AssetType)Data.Read<byte>(0);
            set => Data.Set<byte>(0, (byte)value);
        }
    }
}
```

> Now every asset knows its type at runtime by inspecting `AssetType`.

---

## 5️⃣ Step 5: Create PenguinAsset

1. At **Assets/Scripts/** (or under **GameEngine/** if preferred), create **PenguinAsset.cs**.
2. Implement the Player asset:

```csharp
using Ajuna.SAGE.Core;
using Ajuna.SAGE.Core.Model;
using UnityEngine;

namespace SageUnityLib
{
    /// <summary>
    /// Penguin Player asset with health.
    /// </summary>
    public class PenguinAsset : BaseAsset
    {
        public PenguinAsset(uint ownerId, uint initialHealth = 10)
            : base(ownerId)
        {
            AssetType = AssetType.Player;
            Health = 100;
        }

        /// <summary>
        /// Current health of the Penguin.
        /// Stored as a byte allowing a range of 0 - 255.
        /// </summary>
        public byte Health
        {
            get => Data.Read<byte>(1);
            set => Data.Set<byte>(1, value);
        }
    }
}
```

---

## 6️⃣ Step 6: Instantiate in Scene

1. Open **Assets/Scenes/MainScene.unity** (or create a new scene).
2. In **Hierarchy**, right‑click → **Create Empty** → rename to `Penguin`.
3. Add a new script component:
   - Click **Add Component** on the `Penguin` GameObject.
   - Select **New Script** → name `PenguinSpawner.cs`.
4. Edit **PenguinSpawner.cs**:

```csharp
using UnityEngine;
using SageUnityLib;

public class PenguinSpawner : MonoBehaviour
{
    private PenguinAsset _penguin;

    void Start()
    {
        // Create a new Penguin asset
        _penguin = new PenguinAsset(1); // we use for now ownerId = 1
        Debug.Log($"Spawned Penguin with Health: {_penguin.Health}");
    }
}
```

![Unity Console](https://github.com/ajuna-network/sage-playground/blob/tutorial/tutorial/game_dev/docs/images/Screenshot%202025-06-12%20101406.png?raw=true)

---

## 7️⃣ (Optional) Step 7: Display Health in UI

1. **UI Setup**:
   - In **Hierarchy**, right‑click → **UI → Canvas**.
     - Under the Canvas, right‑click → **UI → Text** (or **TextMeshPro**).
     - Name it `PenguinHealthText` and position it.
   - In **Hierarchy**, right‑click → **UI → Canvas**.
     - Under the Canvas, right‑click → **UI → Image**
     - Name it `PenguinImage` and position it.
     - Import your favorite `2D Penguin Image` into the Unity Project
       - Set the `PenguinImage`'s `Texture Type` to `Sprite (2D and UI)`
       - Set the `PenguinImage`'s `Sprite Mode` to `Single`
     - Drag the imported `2D Penguin Image` to the `Source Image` field of `PenguinImage`

2. **Update PenguinSpawner.cs** in `Assets/Scripts/`:

```csharp
using UnityEngine;
using UnityEngine.UI;
using TMPro;
using SageUnityLib;

public class PenguinSpawner : MonoBehaviour
{
    private PenguinAsset _penguin;
    
    [SerializeField]
    private Image _penguinAvatarImg;
    
    [SerializeField]
    private TMP_Text _penguinHealthTxt;
    void Start()
    {
        // Create a new Penguin asset
        _penguin = new PenguinAsset(1); // we use for now ownerId = 1
        Debug.Log($"Spawned Penguin with Health: {_penguin.Health}");
    }
    
    void Update()
    {
        // Update the health text in the UI
        if (_penguin != null && _penguinAvatarImg != null && _penguinHealthTxt != null)
        {
            // Update the penguin's health
            _penguinAvatarImg.fillAmount = _penguin.Health / 100f; // Assuming Health is between 0 and 100
            _penguinHealthTxt.text = _penguin.Health.ToString();
        }
    }
}
```

3. Drag **Image & Text** to the **Penguin** GameObject’s `PenguinSpawner` inspector slots.
4. Press **Play**. You should see **100** for your Health and a full bar (or your chosen start value).

![Unity Console](https://github.com/ajuna-network/sage-playground/blob/tutorial/tutorial/game_dev/docs/images/Screenshot%202025-06-12%20113746.png?raw=true)

---

## 🟢 Git Tutorial Checkpoints

| Stage      | Commit Description                  | Git Ref                 |
| ---------- | ----------------------------------- | ----------------------- |
| 🟢 Start   | SAGE.dll integrated, project compiles      | [0915430547bde0591555f5f672ec7f708aec98d2](https://github.com/ajuna-network/sage-playground/commit/0915430547bde0591555f5f672ec7f708aec98d2)    |
| ✅ Complete | Penguin asset created and displayed | [30b56f4a993175677f09578e36712dd39bba918b](https://github.com/ajuna-network/sage-playground/commit/30b56f4a993175677f09578e36712dd39bba918b) |

Congratulations! You’ve defined a SAGE-backed **Player** asset in Unity and displayed its health. Next up: create the **Fish** consumable, define the **EAT** transition, and hook it into your scene!

