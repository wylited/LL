using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class ARTouch : MonoBehaviour
{


    public GameObject earthPopup;
    public GameObject marsPopup;
    // Start is called before the first frame update
    void Start()
    {
        
    }

    // Update is called once per frame
    void Update()
    {
        if (Input.GetMouseButtonDown(0))
        {
            Debug.Log("Pressed primary button.");

            Ray ray = Camera.main.ScreenPointToRay(Input.mousePosition);
            RaycastHit hit;
            if (Physics.Raycast(ray, out hit, 100))
            {
                Debug.Log("hit");
                Debug.Log(hit.transform.name + " : " + hit.transform.tag);

                if (hit.transform.tag == "Celestial")
                {
                    Vector3 pos = hit.point;
                    pos.z += 0.25f;
                    pos.y += 0.25f;
                    Instantiate(marsPopup, pos, transform.rotation);
                }

                if (hit.transform.tag == "earth")
                {
                    Vector3 pos = hit.point;
                    pos.z += 0.25f;
                    pos.y += 0.25f;
                    Instantiate(earthPopup, pos, transform.rotation);
                }

                if (hit.transform.tag == "marsInfo")
                {
                    Destroy(hit.transform.gameObject);
                }

                if (hit.transform.tag == "earthInfo")
                {
                    Destroy(hit.transform.gameObject);
                }

            }
        }
    }
}
