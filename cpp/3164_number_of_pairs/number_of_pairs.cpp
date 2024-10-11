#include <cassert>
#include <tuple>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  long long numberOfPairs(vector<int>& nums1, vector<int>& nums2, int k) {
    unordered_map<int, int> count, count2;
    int max1 = 0;
    for (int num : nums1) {
      count[num]++;
      max1 = max(max1, num);
    }
    for (int num : nums2) {
      count2[num]++;
    }
    long long res = 0;
    for (const auto& pair : count2) {
      int a = pair.first, cnt = pair.second;
      for (int b = a * k; b <= max1; b += a * k) {
        if (count.count(b) > 0) {
          res += 1L * count[b] * cnt;
        }
      }
    }
    return res;
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>, int, long long>> tests{
      {{1, 3, 4}, {1, 3, 4}, 1, 5},
      {{1, 2, 4, 12}, {2, 4}, 3, 2},
  };

  for (auto &[nums1, nums2, k, ans] : tests) {
    assert(Solution().numberOfPairs(nums1, nums2, k) == ans);
  }
}
