#include <bits/stdc++.h>
     
    using namespace std;
     
    int main() {
        int n;
        cin >> n;
        int vals[n];
        for (int i = 0; i < n; i++) cin >> vals[i];
     
        long long int left[n];
        long long int lM[n];
        long long int right[n];
        long long int rM[n];

        //the main idea is to calculate how much it costs to make all the vals to the left (left[i]) and to the right (right[i]) of a single index
        //we can then use this to calculate the minimum cost in total
     
        left[0] = 0;
        lM[0] = vals[0];
        //lM effectively serves as a copy to keep track of the necessary changes to the array to make it strictly increasing

        for (long long int i = 1; i < n; i++) {
            if (vals[i] > lM[i-1]) {
                left[i] = left[i-1];
                lM[i] = vals[i];
            } else {
                lM[i] = lM[i-1] + 1;
                left[i] = left[i-1] + ((lM[i-1] + 1) - vals[i]);
            }
        }
     
        right[n-1] = 0;
        rM[n-1] = vals[n-1];
        for (int i = n-2; i >= 0; i--) {
            if (vals[i] > rM[i+1]) {
                right[i] = right[i+1];
                rM[i] = vals[i];
            } else {
                rM[i] = rM[i+1] + 1;
                right[i] = right[i+1] + ((rM[i+1] + 1) - vals[i]);
            }
        }
     
        long long int mn = left[n-1];
     
        for (int i = 0; i < n; i++) {
            // use the precalced vals to find how much it would cost to make the array strict inc/dec to/from a certain index
            // check against min
            long long int sum = 0;
            long long int md = 0;
            if (i != 0) {
                sum += left[i - 1]; 
                md = lM[i - 1];
            }
            if (i != (n-1)) {
                sum += right[i + 1]; 
                md = max(md, rM[i + 1]);
            }
     
            if (md >= vals[i]) sum += (md + 1) - vals[i];
            
            mn = min(mn, sum);
        }
     
        cout << mn;
    }