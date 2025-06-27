using Ajuna.SAGE.Core.Model;
using UnityEngine;
using SageUnityLib;

public class TransitionController : MonoBehaviour
{
  [SerializeField]
  private GameEngine _gameEngine;

  [SerializeField]
  private PenguinSpawner _penguin;

  [SerializeField]
  private FishSpawner _fish;

  public void OnEatButton()
  {
    var inputAssets = new IAsset[] { _penguin.Penguin, _fish.Fish };
    var identifier = GameConfig.Eat(out _, out _);
    Debug.Log($"[OnEatButton] Trying transition: ({identifier.TransitionType}, {identifier.TransitionSubType})");
    // Execute the EAT transition
    var successFlag = _gameEngine.Engine.Transition(
        _gameEngine.User, // User account
        new GameIdentifier((byte)GameAction.Eat),
        inputAssets,
        out IAsset[] outAssets
    );

    if (!successFlag)
    {
      Debug.LogError("Transition failed: " + (GameAction)identifier.TransitionType);
      return;
    }
    else
    {
      _penguin.Penguin = outAssets[0] as PenguinAsset;
      Destroy(_fish._fishAvatarImg.gameObject);
      Destroy(_fish._fishHealthTxt.gameObject);
      Destroy(_fish.gameObject); // optional: only if you want to remove the spawner
    }

    Debug.Log("Transition succeed: " + (GameAction)identifier.TransitionType);

  }
}