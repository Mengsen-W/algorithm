/*
 * @Date: 2023-05-17
 * @LastEditors: 854284842@qq.com
 * @LastEditTime: 2023-05-17
 * @FilePath: /algorithm/cpp/2446_have_conflict/have_conflict.cpp
 */

#include <cassert>
#include <string>
#include <vector>

using namespace std;

class Solution {
 public:
  bool haveConflict(vector<string>& event1, vector<string>& event2) {
    return !(event1[1] < event2[0] || event2[1] < event1[0]);
  }
};

int main() {
  {
    vector<string> event1{"01:15", "02:00"};
    vector<string> event2 = {"02:00", "03:00"};
    bool ans = true;
    assert(Solution().haveConflict(event1, event2) == ans);
  }

  {
    vector<string> event1{"01:00", "02:00"};
    vector<string> event2 = {"01:20", "03:00"};
    bool ans = true;
    assert(Solution().haveConflict(event1, event2) == ans);
  }

  {
    vector<string> event1{"10:00", "11:00"};
    vector<string> event2 = {"14:00", "15:00"};
    bool ans = false;
    assert(Solution().haveConflict(event1, event2) == ans);
  }
}