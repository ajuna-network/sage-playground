using Ajuna.SAGE.Core.Model;
using SageUnityLib;
using UnityEngine;
using SageUnityLib.Model;

public class TransitionController : MonoBehaviour
{
    [SerializeField]
    private GameEngineService _gameEngine;
    
    [SerializeField]
    private PenguinSpawner _penguin;

    [SerializeField]
    private FishSpawner _fish;

    // Start is called once before the first execution of Update after the MonoBehaviour is created
    void Start()
    {

    }

    // Update is called once per frame
    void Update()
    {
        
    }

    public void OnEatButton()
    {
        var inputAssets = new IAsset[] { _penguin.Penguin, _fish.Fish };
        var identifier = new GameIdentifier((byte)GameAction.DoEat);

        // Execute the EAT transition
        var successFlag = _gameEngine.Engine.Transition(
            _gameEngine.User, // User account
            identifier,
            inputAssets,
            out IAsset[] outAssets
        );

        if (!successFlag)
        {
            Debug.LogError("Transition failed: " + (GameAction)identifier.TransitionType);
            return;
        }

        Debug.Log("Transition succeed: " + (GameAction)identifier.TransitionType);

        // Handle the output assets
        _penguin.Penguin = outAssets[0] as PlayerAsset;
        Destroy(_fish.gameObject);
    }
}
