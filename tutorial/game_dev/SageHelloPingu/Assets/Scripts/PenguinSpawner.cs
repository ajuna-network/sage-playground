using UnityEngine;
using SageUnityLib;
using TMPro;
using UnityEngine.UI;

public class PenguinSpawner : MonoBehaviour
{
    public PlayerAsset Penguin { get; set; }

    [SerializeField]
    private GameEngine _gameEngine;

    [SerializeField]
    private Image _playerAvatarImg;

    [SerializeField]
    private TMP_Text _playerHealthTxt;

    void Start()
    {
        // Create a new Penguin asset
        Penguin = new PlayerAsset(_gameEngine.User.Id);
        Debug.Log($"Spawned Penguin with Health: {Penguin.Health}");
    }

    void Update()
    {
        // Update the health text in the UI
        if (Penguin != null && _playerAvatarImg != null && _playerHealthTxt != null)
        {
            // Update the player's health
            _playerAvatarImg.fillAmount = Penguin.Health / 100f; // Assuming Health is between 0 and 100
            _playerHealthTxt.text = Penguin.Health.ToString();
        }
    }
}