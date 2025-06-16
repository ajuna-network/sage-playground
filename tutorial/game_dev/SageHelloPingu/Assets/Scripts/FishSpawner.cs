using UnityEngine;
using SageUnityLib;
using TMPro;

public class FishSpawner : MonoBehaviour
{
    public ConsumableAsset Fish { get; set; }

    [SerializeField]
    private GameEngine _gameEngine;

    [SerializeField]
    private TMP_Text _fishHealthTxt;

    void Start()
    {
        // Create a new fish asset
        Fish = new ConsumableAsset(_gameEngine.User.Id);
        Debug.Log($"Spawned Fish that restores: {Fish.HealthValue} Health");
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