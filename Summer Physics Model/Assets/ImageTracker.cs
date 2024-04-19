using System.Collections;
using System.Collections.Generic;
using UnityEngine;

using UnityEngine.Networking;
using UnityEngine.XR.ARFoundation;
using UnityEngine.XR.ARSubsystems;

public class ImageTracker : MonoBehaviour
{
    [System.Serializable]
    public class ResourcesData
    {
        public List<ResourceData> resources;
    }

    [System.Serializable]
    public class ResourceData
    {
        public string id;
        public long book_isbn;
        public string title;
        public string author;
        public string description;
        public string file_name;
        public int collab_score;
        public int page_number;
    }

    private ARTrackedImageManager trackedImages;
    public GameObject[] ArPrefabs;
    private Renderer[] planeRenderers;

    List<GameObject> ARObjects = new List<GameObject>();

    private IEnumerator DownloadandLoadTextures() {
        Debug.Log("getting textures");
        yield return DownloadAllTextureAsset((textures) => {
            Debug.Log("Got Textures");
            if (textures != null)
            {
                foreach (var texturePair in textures)
                {
                    var pageNumber = "page-0" + texturePair.Key;
                    Debug.Log("Resource on Page Number: " + pageNumber);
                    var texture = texturePair.Value;
                    float ratio = (float)texture.width / texture.height;
                    Debug.Log("Texture ratio: " + ratio);

                    if (texture != null && 14 <= texturePair.Key && 22 >= texturePair.Key ) {
                        Renderer planeRenderer = planeRenderers[texturePair.Key - 12];
                        planeRenderer.sharedMaterial.mainTexture = texture;
                        Vector3 scale = new Vector3(0.008f * ratio, 0.008f, 0.008f);
                        planeRenderer.transform.localScale = scale;
                    }
                    else {
                        Debug.LogError($"Failed to download texture for page {pageNumber}.");
                    }
                }
                Debug.Log("Finished going through textures");
            }
            else
            {
                Debug.LogError("Failed to download textures.");
            }
        });

    }


    public static IEnumerator DownloadAllTextureAsset(System.Action<Dictionary<int, Texture>> callback)
    {
        using (var request = UnityWebRequest.Get($"http://188.166.250.75:3000/api/resources/9781009071888/unity"))
        {
            yield return request.SendWebRequest();
            if (request.result == UnityWebRequest.Result.Success)
            {
                var textures = new Dictionary<int, Texture>();
                var resources = JsonUtility.FromJson<ResourcesData>(request.downloadHandler.text);
                foreach (var resource in resources.resources)
                {
                    using (var req2 = UnityWebRequestTexture.GetTexture($"http://188.166.250.75:3000/api/resources/9781009071888/{resource.id}/image"))
                    {
                        yield return req2.SendWebRequest();

                        if (req2.result == UnityWebRequest.Result.Success)
                        {
                            textures[resource.page_number] = ((DownloadHandlerTexture)req2.downloadHandler).texture;
                        }
                        else
                        {
                            Debug.Log($"Error downloading texture: {req2.error}");
                        }
                    }
                }
                callback(textures);
            }
            else
            {
                Debug.LogError($"Error downloading resources: {request.error}");
            }
        }
    }

    void Awake() {
        planeRenderers = new Renderer[ArPrefabs.Length];
        for (int i = 0; i < ArPrefabs.Length; i++) {
            planeRenderers[i] = ArPrefabs[i].GetComponent<Renderer>();
        }
        trackedImages = GetComponent<ARTrackedImageManager>();
        StartCoroutine(DownloadandLoadTextures());
    }

    void Update()
    {
        outputTracking();
    }

    void outputTracking()
    {
        int i = 0; //counter

        foreach(var trackedImage in trackedImages.trackables)
        {
            if (trackedImage.trackingState == TrackingState.Limited)
            {
                ARObjects[i].SetActive(false);
            }
            if (trackedImage.trackingState == TrackingState.Tracking)
            {
                ARObjects[i].SetActive(true);
            }
            i++;
        }
    }

    void OnEnable()
    {
        trackedImages.trackedImagesChanged += OnTrackedImagesChanged;
    }

    void OnDisable()
    {
        trackedImages.trackedImagesChanged -= OnTrackedImagesChanged;
    }


    // Event Handler
    private void OnTrackedImagesChanged(ARTrackedImagesChangedEventArgs eventArgs)
    {
        //Create object based on image tracked
        foreach (var trackedImage in eventArgs.added)
        {
            foreach (var arPrefab in ArPrefabs)
            {
                if (trackedImage.referenceImage.name == arPrefab.name)
                {
                    var newPrefab = Instantiate(arPrefab, trackedImage.transform);
                    ARObjects.Add(newPrefab);
                }
            }
        }

        //Update tracking position
        foreach (var trackedImage in eventArgs.updated)
        {
            foreach (var gameObject in ARObjects)
            {
                if (gameObject.name == trackedImage.name)
                {
                    gameObject.SetActive(trackedImage.trackingState == TrackingState.Tracking);
                }
            }
        }

    }
}
