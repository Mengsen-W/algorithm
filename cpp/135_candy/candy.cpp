#include <cassert>
#include <tuple>
#include <vector>

using namespace std;

class Solution {
public:
    int candy(vector<int>& ratings) {
        int n = ratings.size();
        int ret = 1;
        int inc = 1, dec = 0, pre = 1;
        for (int i = 1; i < n; i++) {
            if (ratings[i] >= ratings[i - 1]) {
                dec = 0;
                pre = ratings[i] == ratings[i - 1] ? 1 : pre + 1;
                ret += pre;
                inc = pre;
            } else {
                dec++;
                if (dec == inc) {
                    dec++;
                }
                ret += dec;
                pre = 1;
            }
        }
        return ret;
    }
};

int main() {
  vector<tuple<vector<int>, int>> tests {
    {{1,0,2}, 5},
    {{1,2,2}, 4},
  };

  for (auto &[ratings, ans] : tests) {
    assert(Solution().candy(ratings) == ans);
  }
}