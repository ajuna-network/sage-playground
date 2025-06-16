using UnityEngine;
using SageUnityLib;
using TMPro;
using UnityEngine.UI;
using Ajuna.SAGE.Core.Model;

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
        var ok = _gameEngine.Engine.Transition(_gameEngine.User, new GameIdentifier((byte)GameAction.CreatePenguin), null, out IAsset[] outAssets);
        if (!ok)
        {
            Debug.Log($"Failed to spawn penguin!");
            return;
        }

        // Assign newly created penguin asset
        Penguin = outAssets[0] as PlayerAsset;
        Debug.Log($"Created Penguin (Health: {Penguin.Health}, Genesis: {Penguin.Genesis})");
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