#include <cassert>
#include <numeric>
#include <tuple>
#include <vector>

using namespace  std;

class Solution {
public:
    vector<int> replaceNonCoprimes(vector<int>& nums) {
        vector<int> ans;
        for (int num: nums) {
            while (!ans.empty()) {
                int g = gcd(ans.back(), num);
                if (g > 1) {
                    num = ans.back() / g * num;
                    ans.pop_back();
                }
                else {
                    break;
                }
            }
            ans.push_back(num);
        }
        return ans;
    }
};

int main() {
  vector<tuple<vector<int>, vector<int>>> tests{
      {{6, 4, 3, 2, 7, 6, 2}, {12, 7, 6}},
      {{2, 2, 1, 1, 3, 3, 3}, {2, 1, 1, 3}},
  };

  for (auto &[input, expected] : tests) {
    assert(Solution().replaceNonCoprimes(input) == expected);
  }
}