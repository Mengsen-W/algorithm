#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> occurrencesOfElement(vector<int>& nums, vector<int>& queries, int x) {
    vector<int> indices;
    for (int i = 0; i < nums.size(); i++) {
      if (nums[i] == x) {
        indices.emplace_back(i);
      }
    }
    vector<int> res;
    for (int q : queries) {
      if (indices.size() < q) {
        res.emplace_back(-1);
      } else {
        res.emplace_back(indices[q - 1]);
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>, int, vector<int>>> tests{
      {{1, 3, 1, 7}, {1, 3, 2, 4}, 1, {0, -1, 2, -1}},
      {{1, 2, 3}, {10}, 5, {-1}},
  };
  
  for (auto &[nums, queries, x, ans] : tests) {
    assert(Solution().occurrencesOfElement(nums, queries, x) == ans);
  }
}