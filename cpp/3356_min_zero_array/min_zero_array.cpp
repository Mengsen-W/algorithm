#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  int minZeroArray(vector<int>& nums, vector<vector<int>>& queries) {
    int left = 0, right = queries.size();
    if (!check(nums, queries, right)) {
      return -1;
    }
    while (left < right) {
      int k = (left + right) / 2;
      if (check(nums, queries, k)) {
        right = k;
      } else {
        left = k + 1;
      }
    }
    return left;
  }

 private:
  bool check(vector<int>& nums, vector<vector<int>>& queries, int k) {
    vector<int> deltaArray(nums.size() + 1, 0);
    for (int i = 0; i < k; ++i) {
      int left = queries[i][0];
      int right = queries[i][1];
      int value = queries[i][2];
      deltaArray[left] += value;
      deltaArray[right + 1] -= value;
    }
    vector<int> operationCounts;
    int currentOperations = 0;
    for (int delta : deltaArray) {
      currentOperations += delta;
      operationCounts.push_back(currentOperations);
    }
    for (int i = 0; i < nums.size(); ++i) {
      if (operationCounts[i] < nums[i]) {
        return false;
      }
    }
    return true;
  }
};

int main() {
  vector<tuple<vector<int>, vector<vector<int>>, int>> tests{
      {{2, 0, 2}, {{0, 2, 1}, {0, 2, 1}, {1, 1, 3}}, 2},
      {{4, 3, 2, 1}, {{1, 3, 2}, {0, 2, 1}}, -1},
  };

  for (auto& [queries, k, ans] : tests) {
    assert(Solution().minZeroArray(queries, k) == ans);
  }
}