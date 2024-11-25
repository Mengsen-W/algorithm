#include <cassert>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<int> smallestRange(vector<vector<int>>& nums) {
    int n = nums.size();
    unordered_map<int, vector<int>> indices;
    int xMin = INT_MAX, xMax = INT_MIN;
    for (int i = 0; i < n; ++i) {
      for (const int& x : nums[i]) {
        indices[x].push_back(i);
        xMin = min(xMin, x);
        xMax = max(xMax, x);
      }
    }

    vector<int> freq(n);
    int inside = 0;
    int left = xMin, right = xMin - 1;
    int bestLeft = xMin, bestRight = xMax;

    while (right < xMax) {
      ++right;
      if (indices.count(right)) {
        for (const int& x : indices[right]) {
          ++freq[x];
          if (freq[x] == 1) {
            ++inside;
          }
        }
        while (inside == n) {
          if (right - left < bestRight - bestLeft) {
            bestLeft = left;
            bestRight = right;
          }
          if (indices.count(left)) {
            for (const int& x : indices[left]) {
              --freq[x];
              if (freq[x] == 0) {
                --inside;
              }
            }
          }
          ++left;
        }
      }
    }

    return {bestLeft, bestRight};
  }
};

int main() {
  vector<tuple<vector<vector<int>>, vector<int>>> tests{
      {{{4, 10, 15, 24, 26}, {0, 9, 12, 20}, {5, 18, 22, 30}}, {20, 24}},
      {{{1, 2, 3}, {1, 2, 3}, {1, 2, 3}}, {1, 1}},
  };

  for (auto &[nums, ans] : tests) {
    assert(Solution().smallestRange(nums) == ans);
  }
}