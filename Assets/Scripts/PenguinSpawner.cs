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