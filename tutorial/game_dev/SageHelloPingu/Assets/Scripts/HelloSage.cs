using UnityEngine;
using System;
using System.Reflection;

public class HelloSage : MonoBehaviour
{
    // Start is called once before the first execution of Update after the MonoBehaviour is created
    void Start()
    {
        // Retrieve and log the Ajuna.SAGE.Core assembly version
        Assembly sageCoreAssembly = typeof(Ajuna.SAGE.Core.Utils).Assembly;
        Version sageVersion = sageCoreAssembly.GetName().Version;
        Debug.Log($"Ajuna.SAGE.Core version: {sageVersion}");
    }

}
