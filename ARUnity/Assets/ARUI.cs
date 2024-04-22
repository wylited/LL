using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using TMPro;
using UnityEngine.UI;



public class ARUI : MonoBehaviour
{

    public Canvas canvas;

    bool earthScaled = false;
    Vector3 earthScaleOg;




    int infoPointer = -1;

    // Start is called before the first frame update
    void Start()
    {
        canvas.enabled = false;
    }

    // Update is called once per frame
    void Update()
    {
        if (Input.GetMouseButtonDown(0))//left mouse click
        {
            Ray ray = Camera.main.ScreenPointToRay(Input.mousePosition);
            RaycastHit hit;

            if (Physics.Raycast(ray, out hit, 50))
            {
                if (hit.transform.tag == "Celestial")
                {
                    //do something!
                    displayCanvas();
                    infoPointer = 0;
                    if (earthScaled == false)
                    {
                        earthScaled = true;
                        earthScaleOg = hit.transform.localScale;
                        Vector3 scale = earthScaleOg * 1f;
                        hit.transform.localScale = scale;
                    }
                }
                else
                {
                    if (earthScaled == true)
                    {
                        GameObject temp = GameObject.FindGameObjectWithTag("Celestial");
                        temp.transform.localScale = earthScaleOg;
                        earthScaled = false;
                    }
                }
            }
        }
    }
                public void displayCanvas()
                {
                    canvas.enabled = true;

                }

                public void hideCanvas()
                {
                    canvas.enabled = false;
                }
            
}
