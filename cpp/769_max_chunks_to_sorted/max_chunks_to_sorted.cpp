/*
 * @Date: 2022-10-13
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-13
 * @FilePath: /algorithm/769_max_chunks_to_sorted/max_chunks_to_sorted.cpp
 */

#include <cassert>
#include <vector>

using namespace std;

class Solution {
 public:
  int maxChunksToSorted(vector<int>& arr) {
    int m = 0, res = 0;
    for (int i = 0; i < arr.size(); i++) {
      m = max(m, arr[i]);
      if (m == i) {
        res++;
      }
    }
    return res;
  }
};

int main() {
  {
    vector<int> arr{4, 3, 2, 1, 0};
    int ans = 1;
    assert(Solution().maxChunksToSorted(arr) == ans);
  }

  {
    vector<int> arr{1, 0, 2, 3, 4};
    int ans = 4;
    assert(Solution().maxChunksToSorted(arr) == ans);
  }
}
