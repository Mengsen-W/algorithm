/*
 * @Date: 2021-07-26 10:38:44
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-07-26 12:48:40
 */

#include <cassert>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  int minOperations(vector<int> &target, vector<int> &arr) {
    int n = target.size();
    unordered_map<int, int> pos;
    for (int i = 0; i < n; ++i) {
      pos[target[i]] = i;
    }
    vector<int> d;
    for (int val : arr) {
      if (pos.count(val)) {
        int idx = pos[val];
        auto it = lower_bound(d.begin(), d.end(), idx);
        if (it != d.end()) {
          *it = idx;
        } else {
          d.push_back(idx);
        }
      }
    }
    return n - d.size();
  }
};

int main() {
  {
    vector<int> target{5, 1, 3};
    vector<int> arr{9, 4, 2, 3, 4};
    assert(Solution{}.minOperations(target, arr) == 2);
  }

  {
    vector<int> target{5, 1, 3};
    vector<int> arr{9, 4, 2, 3, 4};
    assert(Solution{}.minOperations(target, arr) == 2);
  }

  return 0;
}