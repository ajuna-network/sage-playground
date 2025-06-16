using Ajuna.SAGE.Core.Model;

namespace SageUnityLib
{
    /// <summary>
    /// Represents a game identifier used in the game engine transitions.
    /// </summary>
    public struct GameIdentifier : ITransitionIdentifier
    {
        public byte TransitionType { get; set; }
        public byte TransitionSubType { get; set; }

        public GameIdentifier(byte transitionType, byte transitionSubType)
        {
            TransitionType = transitionType;
            TransitionSubType = transitionSubType;
        }

        public GameIdentifier(byte transitionType) : this(transitionType, 0)
        {
        }

        public static GameIdentifier CreatePenguin => new GameIdentifier((byte)GameAction.CreatePenguin);
        public static GameIdentifier CreateFish => new GameIdentifier((byte)GameAction.CreateFish);
        public static GameIdentifier DoEat => new GameIdentifier((byte)GameAction.DoEat);
    }
}