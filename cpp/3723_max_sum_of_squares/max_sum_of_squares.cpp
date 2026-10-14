#include <cassert>
#include <string>
#include <tuple>
#include <vector>
using namespace std;


class Solution {
public:
 string maxSumOfSquares(int num, int sum) {
   if (sum > 9 * num) {
     return "";
   }
   int r = sum % 9;
   string ans(sum / 9, '9');
   if (r > 0) {
     ans.push_back('0' + r);
   }
   ans.append(num - (int)ans.size(), '0');
   return ans;
 }
};

int main() {
  vector<tuple<int, int, string>> tests{
      {2, 3, "30"},
      {2, 17, "98"},
      {1, 10, ""},
  };

  for (auto [num, sum, expected] : tests) {
    assert(Solution().maxSumOfSquares(num, sum) == expected);
  }
}