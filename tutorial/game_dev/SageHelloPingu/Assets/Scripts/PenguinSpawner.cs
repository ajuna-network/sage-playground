using UnityEngine;
using SageUnityLib;
using TMPro;
using UnityEngine.UI;

public class PenguinSpawner : MonoBehaviour
{
    private PlayerAsset _player;

    [SerializeField]
    private Image _playerAvatarImg;

    [SerializeField]
    private TMP_Text _playerHealthTxt;

    void Start()
    {
        // Create a new Penguin asset
        _player = new PlayerAsset(1); // we use for now ownerId = 1
        Debug.Log($"Spawned Penguin with Health: {_player.Health}");
    }

    void Update()
    {
        // Update the health text in the UI
        if (_player != null && _playerAvatarImg != null && _playerHealthTxt != null)
        {
            // Update the player's health
            _playerAvatarImg.fillAmount = _player.Health / 100f; // Assuming Health is between 0 and 100
            _playerHealthTxt.text = _player.Health.ToString();
        }
    }
}