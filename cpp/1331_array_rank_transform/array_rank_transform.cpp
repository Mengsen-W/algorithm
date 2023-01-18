/*
 * @Date: 2022-07-28
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-07-28
 * @FilePath: /algorithm/1331_array_rank_transform/array_rank_transform.cpp
 */

#include <cassert>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> arrayRankTransform(vector<int>& arr) {
    vector<int> sortedArr = arr;
    sort(sortedArr.begin(), sortedArr.end());
    unordered_map<int, int> ranks;
    vector<int> ans(arr.size());
    for (auto& a : sortedArr) {
      if (!ranks.count(a)) {
        ranks[a] = ranks.size() + 1;
      }
    }
    for (int i = 0; i < arr.size(); i++) {
      ans[i] = ranks[arr[i]];
    }
    return ans;
  }
};

int main() {
  {
    vector<int> arr{40, 10, 20, 30};
    vector<int> ans{4, 1, 2, 3};
    assert(Solution().arrayRankTransform(arr) == ans);
  }

  {
    vector<int> arr{100, 100, 100};
    vector<int> ans{1, 1, 1};
    assert(Solution().arrayRankTransform(arr) == ans);
  }

  {
    vector<int> arr{37, 12, 28, 9, 100, 56, 80, 5, 12};
    vector<int> ans{5, 3, 4, 2, 8, 6, 7, 1, 3};
    assert(Solution().arrayRankTransform(arr) == ans);
  }
}