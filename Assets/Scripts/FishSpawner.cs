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