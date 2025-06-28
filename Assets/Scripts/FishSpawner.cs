using Ajuna.SAGE.Core.Model;
using UnityEngine;
using UnityEngine.UI;
using TMPro;
using SageUnityLib;

public class FishSpawner : MonoBehaviour
{
  public FishAsset Fish { get; set; }

  [SerializeField]
  public Image _fishAvatarImg;

  [SerializeField]
  public TMP_Text _fishHealthTxt;

  [SerializeField]
  private GameEngine _gameEngine;

  void Awake()
  {
    // Create a new Penguin asset
    // Fish = new FishAsset(_gameEngine.User.Id);
    // Debug.Log($"Spawned Fish that restores: {Fish.HealthValue} Health");
  }

	void Start()
	{
  	  bool ok = _gameEngine.Engine.Transition(
  	      _gameEngine.User,
  	      new GameIdentifier((byte)GameAction.CreateFish),
  	      null,
  	      out IAsset[] outAssets
 	   );
 	   if (!ok) { Debug.LogError("Failed to create Fish"); return; }

 	   Fish = outAssets[0] as FishAsset;
 	   Debug.Log($"Created Fish (Value: {Fish.HealthValue}, Genesis: {Fish.Genesis})");
  }

  void Update()
  {
    // Update the health text in the UI
    if (Fish != null && _fishAvatarImg != null && _fishHealthTxt != null)
    {
      // Update the fish's health
      _fishAvatarImg.fillAmount = Fish.HealthValue / 100f; // Assuming Health is between 0 and 100
      _fishHealthTxt.text = Fish.HealthValue.ToString();
    }
  }
}