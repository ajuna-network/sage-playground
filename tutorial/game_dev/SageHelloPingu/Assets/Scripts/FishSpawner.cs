using UnityEngine;
using SageUnityLib;
using TMPro;
using Ajuna.SAGE.Core.Model;

public class FishSpawner : MonoBehaviour
{
    public ConsumableAsset Fish { get; set; }

    [SerializeField]
    private GameEngine _gameEngine;

    [SerializeField]
    private TMP_Text _fishHealthTxt;

    void Start()
    {
        var ok = _gameEngine.Engine.Transition(_gameEngine.User, new GameIdentifier((byte)GameAction.CreateFish), null, out IAsset[] outAssets);
        if (!ok)
        { 
            Debug.LogError("Failed to create Fish"); 
            return; 
        }

        // Assign newly created fish asset
        Fish = outAssets[0] as ConsumableAsset;
        Debug.Log($"Created Fish (Value: {Fish.HealthValue}, Genesis: {Fish.Genesis})");
    }

    void Update()
    {
        // Update the health value text in the UI
        if (Fish != null && _fishHealthTxt != null)
        {
            _fishHealthTxt.text = $"{Fish.HealthValue} HP";
        }
    }

    private void OnMouseDown()
    {
        Debug.Log("Fish clicked!");

        // Example interaction: Consume the fish or increase health
        if (Fish != null)
        {
            Debug.Log($"You clicked the fish. It gives {Fish.HealthValue} health!");
            // Do something like apply the health or destroy the fish
            // Destroy(gameObject);
        }
    }

}