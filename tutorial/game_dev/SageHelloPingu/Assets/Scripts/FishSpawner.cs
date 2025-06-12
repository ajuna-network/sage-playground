using UnityEngine;
using SageUnityLib;
using TMPro;

public class FishSpawner : MonoBehaviour
{
    private ConsumableAsset _fish;

    [SerializeField]
    private TMP_Text _fishHealthTxt;

    void Start()
    {
        // Create a new fish asset
        _fish = new ConsumableAsset(1); // we use for now ownerId = 1
        Debug.Log($"Spawned Fish that restores: {_fish.HealthValue} Health");
    }

    void Update()
    {
        // Update the health value text in the UI
        if (_fish != null && _fishHealthTxt != null)
        {
            _fishHealthTxt.text = $"{_fish.HealthValue} HP";
        }
    }
}