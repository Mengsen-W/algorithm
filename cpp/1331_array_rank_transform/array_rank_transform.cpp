#include <algorithm>
#include <cassert>
#include <tuple>
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
  vector<tuple<vector<int>, vector<int>>> tests{
      {{40, 10, 20, 30}, {4, 1, 2, 3}},
      {{100, 100, 100}, {1, 1, 1}},
      {{37, 12, 28, 9, 100, 56, 80, 5, 12}, {5, 3, 4, 2, 8, 6, 7, 1, 3}},
  };

  for (auto& [arr, ans] : tests) {
    assert(Solution().arrayRankTransform(arr) == ans);
  }
}