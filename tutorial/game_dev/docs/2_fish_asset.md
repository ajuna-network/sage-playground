# Unity Consumable Asset (Fish) Setup

This guide shows you how to define and display a **Consumable** asset (Fish) in Unity using the `Ajuna.SAGE.Core` reference asset structure. The Fish will restore health when used by the Penguin.

---

## 📂 Folder Structure

```
YourUnityProject/
│
└── Assets/
    ├── Scripts/
    │   ├── GameEngine/            # Core game logic library
    │   │   ├── BaseAsset.cs
    │   │   ├── Enum.cs            # AssetType enum (Player, Consumable)
    │   │   ├── PenguinAsset.cs     # Player asset
    │   │   └── FishAsset.cs       # Consumable asset
    │   ├── PenguinSpawner.cs      # Spawns and displays Penguin
    │   └── FishSpawner.cs         # Spawns and displays Fish
    ├── Plugins/                   # Ajuna.SAGE.Core DLLs
    └── Scenes/                    # Unity scenes
        └── MainScene.unity        # Example scene with Penguin & Fish
```

---

## 🎯 Goal

1. Create a **FishAsset** class inheriting **BaseAsset**.
2. Add properties: `AssetType = Consumable`, `HealthValue` (uint).
3. Instantiate a Fish asset in the scene.
4. Optionally, differentiate its visual representation in Unity.

---

## 1️⃣ Step 1: Define `FishAsset` Class

1. In **Assets/Scripts/GameEngine**, create **FishAsset.cs**.
2. Add the following code:

```csharp
using Ajuna.SAGE.Core;
using Ajuna.SAGE.Core.Model;

namespace SageUnityLib
{
    /// <summary>
    /// Fish consumable asset that restores health.
    /// </summary>
    public class FishAsset : BaseAsset
    {
        /// <summary>
        /// Constructs a Fish with a specified health value.
        /// </summary>
        public FishAsset(uint ownerId, uint healthValue = 5)
            : base(ownerId)
        {
            AssetType = AssetType.Consumable;
            HealthValue = 5;
        }

        /// <summary>
        /// Current health returned on consumation of the Asset.
        /// Stored as a byte allowing a range of 0 - 255.
        /// </summary>
        public byte HealthValue
        {
            get => Data.Read<byte>(1);
            set => Data.Set(1, value);
        }
    }
}
```

> **Note:** We reuse the first data byte (index 0) for `AssetType` and the byte for `HealthValue`.

---

## 2️⃣ Step 2: Spawn Fish in the Scene

1. Open **MainScene.unity** (or your working scene).
2. In the **Hierarchy**, create an `Empty GameObject` and name it `Sardine`.
3. Add a new Script component to the newly created `Sardine` GameObject:
   - Click **Add Component** → **New Script** → name `FishSpawner`.
4. Edit **FishSpawner.cs**:

```csharp
using UnityEngine;
using UnityEngine.UI;
using TMPro;
using SageUnityLib;

public class FishSpawner : MonoBehaviour
{
    private FishAsset _fish;

    void Start()
    {
        // Create a new Penguin asset
        _fish = new FishAsset(1); // we use for now ownerId = 1
        Debug.Log($"Spawned Fish that restores: {_fish.HealthValue} Health");
    }
    
}
```

---

## 3️⃣ Step 3: Visual Differentiation
1. **UI Setup**:
   - In **Hierarchy**, right‑click → **UI → Canvas**.
      - Under the Canvas, right‑click → **UI → Text** (or **TextMeshPro**).
      - Name it `SardineHealthText` and position it.
   - In **Hierarchy**, right‑click → **UI → Canvas**.
     - Under the Canvas, right‑click → **UI → Image**
     - Name it `SardineImage` and position it.
     - Import your favorite `2D Sardine Image` into the Unity Project
        - Set the `SardineImage`'s `Texture Type` to `Sprite (2D and UI)`
        - Set the `SardineImage`'s `Sprite Mode` to `Single`
     - Drag the imported `2D Sardine Image` to the `Source Image` field of `SardineImage`

2. **Update FishSpawner.cs** in `Assets/Scripts/`:

```csharp
using UnityEngine;
using UnityEngine.UI;
using TMPro;
using SageUnityLib;

public class FishSpawner : MonoBehaviour
{
    private FishAsset _fish;
    
    [SerializeField]
    private Image _fishAvatarImg;
    
    [SerializeField]
    private TMP_Text _fishHealthTxt;
    
    void Start()
    {
        // Create a new Penguin asset
        _fish = new FishAsset(1); // we use for now ownerId = 1
        Debug.Log($"Spawned Fish that restores: {_fish.HealthValue} Health");
    }
    
    void Update()
    {
        // Update the health text in the UI
        if (_fish != null && _fishAvatarImg != null && _fishHealthTxt != null)
        {
            // Update the fish's health
            _fishAvatarImg.fillAmount = _fish.HealthValue / 100f; // Assuming Health is between 0 and 100
            _fishHealthTxt.text = _fish.HealthValue.ToString();
        }
    }
}
```

3. Drag **Image & Text** to the **Fish** GameObject’s `FishSpawner` inspector slots.
4. Press **Play**. You should see **5** for your `Sardine`'s Health (or your chosen start value).

![Unity Console](https://github.com/ajuna-network/sage-playground/blob/tutorial/tutorial/game_dev/docs/images/Screenshot%202025-06-12%20121159.png?raw=true)

---

## 🟢 Git Tutorial Checkpoints

| Stage      | Commit Description                     | Git Ref                 |
| ---------- | -------------------------------------- | ----------------------- |
| 🟢 Start   | Before Fish asset is added             | [ff5c0831399646de87f813ad0f226fd195a88715](https://github.com/ajuna-network/sage-playground/commit/ff5c0831399646de87f813ad0f226fd195a88715) |
| ✅ Complete | Fish asset created and placed in scene | [a656f6633f5e25f53f4140e2687a0bfd7d748678](https://github.com/ajuna-network/sage-playground/commit/a656f6633f5e25f53f4140e2687a0bfd7d748678) |

> Use:
>
> ```bash
> git checkout replace-this-start
> git checkout replace-this-complete
> ```

---

Congratulations! You’ve defined and instantiated a **Consumable** asset in Unity. Next up: implement the **EAT** transition to wire Penguin and Fish assets together and update state via SAGE.

