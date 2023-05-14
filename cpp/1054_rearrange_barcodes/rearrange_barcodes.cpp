/*
 * @Date: 2023-05-14
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-14
 * @FilePath: /algorithm/cpp/1054_rearrange_barcodes/rearrange_barcodes.cpp
 */

#include <cassert>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> rearrangeBarcodes(vector<int>& barcodes) {
    int length = barcodes.size();
    if (length < 2) {
      return barcodes;
    }

    unordered_map<int, int> counts;
    int maxCount = 0;
    for (int b : barcodes) {
      maxCount = max(maxCount, ++counts[b]);
    }

    int evenIndex = 0, oddIndex = 1, halfLength = length / 2;
    vector<int> res(length);
    for (auto& [x, cx] : counts) {
      while (cx > 0 && cx <= halfLength && oddIndex < length) {
        res[oddIndex] = x;
        cx--;
        oddIndex += 2;
      }
      while (cx > 0) {
        res[evenIndex] = x;
        cx--;
        evenIndex += 2;
      }
    }
    return res;
  }
};

int main() {
  {
    vector<int> barcodes{1, 1, 1, 2, 2, 2};
    vector<int> ans{2, 1, 2, 1, 2, 1};
    assert(Solution().rearrangeBarcodes(barcodes) == ans);
  }

  {
    vector<int> barcodes{1, 1, 1, 1, 2, 2, 3, 3};
    vector<int> ans{2, 1, 2, 1, 3, 1, 3, 1};
    assert(Solution().rearrangeBarcodes(barcodes) == ans);
  }
}