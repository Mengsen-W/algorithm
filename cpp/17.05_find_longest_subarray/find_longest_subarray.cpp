/*
 * @Date: 2023-03-11
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-03-11
 * @FilePath: /algorithm/cpp/17.05_find_longest_subarray/find_longest_subarray.cpp
 */

#include <cassert>
#include <string>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<string> findLongestSubarray(vector<string>& array) {
    unordered_map<int, int> indices;
    indices[0] = -1;
    int sum = 0;
    int maxLength = 0;
    int startIndex = -1;
    int n = array.size();
    for (int i = 0; i < n; i++) {
      if (isalpha(array[i][0])) {
        sum++;
      } else {
        sum--;
      }
      if (indices.count(sum)) {
        int firstIndex = indices[sum];
        if (i - firstIndex > maxLength) {
          maxLength = i - firstIndex;
          startIndex = firstIndex + 1;
        }
      } else {
        indices[sum] = i;
      }
    }
    if (maxLength == 0) {
      return {};
    }
    return vector<string>(array.begin() + startIndex, array.begin() + startIndex + maxLength);
  }
};

int main() {
  {
    vector<string> array{"A", "1", "B", "C", "D", "2", "3", "4", "E", "5",
                         "F", "G", "6", "7", "H", "I", "J", "K", "L", "M"};
    vector<string> ans{"A", "1", "B", "C", "D", "2", "3", "4", "E", "5", "F", "G", "6", "7"};
    assert(Solution().findLongestSubarray(array) == ans);
  }

  {
    vector<string> array{"A", "A"};
    vector<string> ans{};
    assert(Solution().findLongestSubarray(array) == ans);
  }
}