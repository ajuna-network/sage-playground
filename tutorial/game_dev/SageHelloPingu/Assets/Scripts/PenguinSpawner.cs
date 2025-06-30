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