/*
 * @Date: 2022-12-23
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-12-23
 * @FilePath: /algorithm/2011_final_value_after_operations/final_value_after_operations.cpp
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  int finalValueAfterOperations(vector<string>& operations) {
    int x = 0;
    for (auto& op : operations) {
      if (op == "X++" || op == "++X") {
        x++;
      } else {
        x--;
      }
    }
    return x;
  }
};

int main() {
  {
    vector<string> operations{"--X", "X++", "X++"};
    int ans = 1;
    assert(Solution().finalValueAfterOperations(operations) == ans);
  }

  {
    vector<string> operations{"++X", "++X", "X++"};
    int ans = 3;
    assert(Solution().finalValueAfterOperations(operations) == ans);
  }

  {
    vector<string> operations{"X++", "++X", "--X", "X--"};
    int ans = 0;
    assert(Solution().finalValueAfterOperations(operations) == ans);
  }
}