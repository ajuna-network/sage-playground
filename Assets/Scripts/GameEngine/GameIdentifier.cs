using Ajuna.SAGE.Core.Model;
using System;

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