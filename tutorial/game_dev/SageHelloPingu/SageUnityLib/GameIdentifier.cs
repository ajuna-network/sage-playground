using Ajuna.SAGE.Core.Model;
using System;

namespace SageUnityLib.Model
{
    public class GameIdentifier : ITransitionIdentifier
    {

        public static GameIdentifier CreatePenguin => new((byte)GameAction.CreatePenguin);
        public static GameIdentifier CreateFish => new((byte)GameAction.CreateFish);
        public static GameIdentifier Eat => new((byte)GameAction.Eat);

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

        public override bool Equals(object obj)
        {
            if (obj is not GameIdentifier other) return false;
            return TransitionType == other.TransitionType &&
                   TransitionSubType == other.TransitionSubType;
        }

        public override int GetHashCode()
        {
            return HashCode.Combine(TransitionType, TransitionSubType);
        }
    }
}