/*
 * @Date: 2022-10-15
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-10-15
 * @FilePath: /algorithm/1441_build_array/build_array.cpp
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  vector<string> buildArray(vector<int>& target, int n) {
    vector<string> res;
    int prev = 0;
    for (int number : target) {
      for (int i = 0; i < number - prev - 1; i++) {
        res.emplace_back("Push");
        res.emplace_back("Pop");
      }
      res.emplace_back("Push");
      prev = number;
    }
    return res;
  }
};

int main() {
  {
    vector<int> target{1, 3};
    int n = 3;
    vector<string> ans{"Push", "Push", "Pop", "Push"};
    assert(Solution().buildArray(target, n) == ans);
  }

  {
    vector<int> target{1, 2, 3};
    int n = 3;
    vector<string> ans{"Push", "Push", "Push"};
    assert(Solution().buildArray(target, n) == ans);
  }

  {
    vector<int> target{1, 2};
    int n = 4;
    vector<string> ans{"Push", "Push"};
    assert(Solution().buildArray(target, n) == ans);
  }
}