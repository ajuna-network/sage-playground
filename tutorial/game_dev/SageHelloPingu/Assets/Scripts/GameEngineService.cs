using Ajuna.SAGE.Core;
using Ajuna.SAGE.Core.Model;
using SageUnityLib;
using TMPro;
using UnityEngine;

public class GameEngineService : MonoBehaviour
{
    public IBlockchainInfoProvider BlockchainInfoProvider { get; private set; }
    
    public Engine<GameIdentifier, GameRule> Engine { get; private set; }

    public IAccount User { get; private set; }

    [SerializeField]
    private TMP_Text _blockNumberTxt;

    private void Awake()
    {
        BlockchainInfoProvider = new BlockchainInfoProvider(1234);
        Engine = GameEngine.Create(BlockchainInfoProvider);

        // Create a user account and add some balance to it
        User = Engine.AccountManager.Account(Engine.AccountManager.Create());
        User.Balance.Deposit(1_000_000);
    }

    private void Start()
    {
        // update block number
        InvokeRepeating(nameof(UpdatedBlocknumber), 0f, 6f);
    }

    // Update is called once per frame
    private void Update()
    {
    }

    private void UpdatedBlocknumber()
    {
        BlockchainInfoProvider.CurrentBlockNumber++;
        Debug.Log($"Blocknumber: {BlockchainInfoProvider.CurrentBlockNumber}");
        if (_blockNumberTxt != null)   
        {
            _blockNumberTxt.text = $"BLOCK: {BlockchainInfoProvider.CurrentBlockNumber}";
        }
    }
}