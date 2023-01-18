/*
 * @Date: 2022-09-09
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-09-09
 * @FilePath: /algorithm/1598_min_operations/min_operations.cpp
 */

#include <assert.h>

#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  int minOperations(vector<string>& logs) {
    int depth = 0;
    for (auto& log : logs) {
      if (log == "./") {
        continue;
      } else if (log == "../") {
        if (depth > 0) {
          depth--;
        }
      } else {
        depth++;
      }
    }
    return depth;
  }
};

int main() {
  {
    vector<string> logs{"d1/", "d2/", "../", "d21/", "./"};
    int ans = 2;
    assert(Solution().minOperations(logs) == ans);
  }

  {
    vector<string> logs{"d1/", "d2/", "./", "d3/", "../", "d31/"};
    int ans = 3;
    assert(Solution().minOperations(logs) == ans);
  }

  {
    vector<string> logs{"d1/", "../", "../", "../"};
    int ans = 0;
    assert(Solution().minOperations(logs) == ans);
  }
}