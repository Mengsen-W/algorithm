#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
 public:
  bool lemonadeChange(vector<int>& bills) {
    int five = 0, ten = 0;
    for (auto& bill : bills) {
      if (bill == 5) {
        five++;
      } else if (bill == 10) {
        if (five == 0) {
          return false;
        }
        five--;
        ten++;
      } else {
        if (five > 0 && ten > 0) {
          five--;
          ten--;
        } else if (five >= 3) {
          five -= 3;
        } else {
          return false;
        }
      }
    }
    return true;
  }
};

int main() {
  vector<tuple<vector<int>, bool>> tests{
      {{5, 5, 5, 10, 20}, true},
      {{5, 5, 10, 10, 20}, false},
  };

  for (auto& [bills, ans] : tests) {
    assert(Solution().lemonadeChange(bills) == ans);
  }
}
