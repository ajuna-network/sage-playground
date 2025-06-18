# Unity Project Setup with SAGE Integration

This guide walks you through setting up a new Unity project and integrating the SAGE framework (`Ajuna.SAGE.Core.dll`) for asset management and transition logic.

---

## 📦 Prerequisites

- **Unity Version**: Unity LTS 6.x (tested with 6.0.0f1)
- **.NET SDK**: .NET 6.0 or later
- **NuGet Package**: `Ajuna.SAGE.Core` v0.0.6

---

## 🗂️ Folder Structure

```
YourUnityProject/
│
└── Assets/
    ├── Scenes/           # Your Unity scenes (.unity files)
    ├── Scripts/          # C# scripts for game logic
    │   └── HelloSage.cs  # Minimal integration test script
    └── Plugins/          # Third‑party DLLs (SAGE, SDKs, etc.)
        └── Ajuna.SAGE.Core.dll
        └── Ajuna.Core.Generic.dll
        └── SageUnityLib.dll  # Your .NET Core class library
```

> **Tip:** Keep all external DLLs inside `Assets/Plugins/` so Unity automatically recognizes them.

---

## 🚀 Step 1: Create the Unity Project

1. Open Unity Hub and click **New**.
2. Select **Unity 6.x LTS** → **3D** template.
3. Name your project (e.g. `SageHelloPingu`) and choose a location.
4. Click **Create project**.

---

## 🔌 Step 2: Import SAGE.dll into Unity

We recommend using a quick .NET Core class library to pull in **Ajuna.SAGE.Core** and all its dependencies (including **Ajuna.Core.Generic**) and then copying the generated DLLs into Unity.

### Create & Build a .NET Core Class Library (Recommended)

1. **Create a class library** in your solution folder (outside Unity):
   ```bash
   dotnet new classlib -n SageUnityLib -f netstandard2.1
   cd SageUnityLib
   ```
2. **Add the SAGE package** (this will bring in Ajuna.Core.Generic automatically):
   ```bash
   dotnet add package Ajuna.SAGE.Core --version 0.0.6
   ```
3. **Enable local DLL copying** by editing `SageUnityLib.csproj` and adding under `<Project>`:
   ```xml
   <PropertyGroup>
     <!-- Copy all dependent assemblies locally for Unity -->
     <CopyLocalLockFileAssemblies>true</CopyLocalLockFileAssemblies>
   </PropertyGroup>
   ```
4. **Build the library** in Release mode:
   ```bash
   dotnet build --configuration Release
   ```
5. **Copy the generated DLLs** into your Unity project's `Assets/Plugins/` folder:
   - `SageUnityLib.dll` (your wrapper library)
   - `Ajuna.SAGE.Core.dll`
   - `Ajuna.Core.Generic.dll`
   - Any other dependencies (e.g., `System.*.dll` if present)

> Unity will automatically load all `.dll` files located under `Assets/Plugins/`.

![Unity Console](https://github.com/ajuna-network/sage-playground/blob/tutorial/tutorial/game_dev/docs/images/Screenshot%202025-06-12%20093103.png?raw=true)

---

## 🧪 Step 3: Verify SAGE Types in Unity

1. In Unity, create a new C# script under `Assets/Scripts/HelloSage.cs`:
   ```csharp
   using UnityEngine;
   using System;
   using System.Reflection;

   public class HelloSage : MonoBehaviour
   {
        void Start()
        {
            Assembly sageCoreAssembly = typeof(Ajuna.SAGE.Core.Utils).Assembly;
            Version sageVersion = sageCoreAssembly.GetName().Version;
            Debug.Log($"Ajuna.SAGE.Core version: {sageVersion}");
        }
   }
   ```
2. Attach `HelloSage.cs` to any GameObject in your active Scene.
3. Press **Play**. In the Console, you should see:
   ```
   Ajuna.SAGE.Core version: 0.0.6.0
   ```

![Unity Console](https://github.com/ajuna-network/sage-playground/blob/tutorial/tutorial/game_dev/docs/images/Screenshot%202025-06-12%20093010.png?raw=true)

> **Tip:** Replace `Transaction` with any other public type exposed by `Ajuna.SAGE.Core` if desired. The key is calling `typeof(<Type>).Assembly.GetName().Version` to fetch the assembly version.

---

## 📚 Documentation

- **Unity Version**: 6.x LTS
- **Folder Layout**: See above
- **DLL Location**: `Assets/Plugins/`
- **Verification**: Console log of Hello SAGE message and IntelliSense availability of `Ajuna.SAGE.Core` types

Optionally, take a screenshot of your **Project** window showing the `Assets/Plugins` folder:



---

## 🟢 Git Tutorial Checkpoints

| Stage      | Commit Description                    | Git Ref                 |
| ---------- | ------------------------------------- | ----------------------- |
| 🟢 Start   | Empty Unity project initialized       | [8aff0e95f4bb4aef3db50320a4165cc004e3fd22](https://github.com/ajuna-network/sage-playground/commit/8aff0e95f4bb4aef3db50320a4165cc004e3fd22)    |
| ✅ Complete | SAGE.dll integrated, project compiles | [2adba118123b21ed0f6abbd3ea417f7b33ee8a75](https://github.com/ajuna-network/sage-playground/commit/2adba118123b21ed0f6abbd3ea417f7b33ee8a75) |

> Link both commits so team members can easily check out each stage:
>
> ```bash
> git checkout 8aff0e95f4bb4aef3db50320a4165cc004e3fd22
> git checkout 2adba118123b21ed0f6abbd3ea417f7b33ee8a75
> ```

---

Congratulations! You now have a Unity project set up with the SAGE framework. The next step is to create your first Penguin and Fish assets, define transitions, and hook up state updates.

