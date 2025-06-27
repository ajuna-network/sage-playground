using UnityEngine;
using UnityEngine.UI;
using TMPro;
using SageUnityLib;

public class PenguinSpawner : MonoBehaviour
{
  public PenguinAsset Penguin { get; set; }

  [SerializeField]
  private Image _penguinAvatarImg;

  [SerializeField]
  private TMP_Text _penguinHealthTxt;

  [SerializeField]
  private GameEngine _gameEngine;
  void Awake()
  {
    // Create a new Penguin asset
    Penguin = new PenguinAsset(_gameEngine.User.Id); // we use for now ownerId = 1
    Debug.Log($"Spawned Penguin with Health: {Penguin.Health}");
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