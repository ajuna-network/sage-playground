using UnityEngine;
using System;
using System.Reflection;

public class HelloSage : MonoBehaviour
{
    void Start()
    {
        Assembly sageCoreAssembly = typeof(Ajuna.SAGE.Core.Utils).Assembly;
        Version sageVersion = sageCoreAssembly.GetName().Version;
        Debug.Log($"Ajuna.SAGE.Core version: {sageVersion}");
    }

}
