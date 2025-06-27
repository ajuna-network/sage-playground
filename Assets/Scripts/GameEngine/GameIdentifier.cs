using Ajuna.SAGE.Core.Model;

public class GameIdentifier : ITransitionIdentifier
{
    public byte TransitionType { get; set; }
    public byte TransitionSubType { get; set; }

    public GameIdentifier(byte transitionType, byte transitionSubType)
    {
        TransitionType = transitionType;
        TransitionSubType = transitionSubType;
    }

    public GameIdentifier(byte transitionType)
        : this(transitionType, 0)
    {
    }
}