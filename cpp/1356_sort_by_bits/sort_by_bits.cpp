#include <algorithm>
#include <cassert>
#include <tuple>
#include <vector>
using namespace std;

class Solution {
 public:
  vector<int> sortByBits(vector<int>& arr) {
    vector<int> bit(10001, 0);
    for (int i = 1; i <= 10000; ++i) {
      bit[i] = bit[i >> 1] + (i & 1);
    }
    sort(arr.begin(), arr.end(), [&](int x, int y) {
      if (bit[x] < bit[y]) {
        return true;
      }
      if (bit[x] > bit[y]) {
        return false;
      }
      return x < y;
    });
    return arr;
  }
};

int main() {
  vector<tuple<vector<int>, vector<int>>> tests{
      {
          {0, 1, 2, 3, 4, 5, 6, 7, 8},
          {0, 1, 2, 4, 8, 3, 5, 6, 7},
      },
      {
          {1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1},
          {1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024},
      },
      {
          {10000, 10000},
          {10000, 10000},
      },
      {
          {2, 3, 5, 7, 11, 13, 17, 19},
          {2, 3, 5, 17, 7, 11, 13, 19},
      },
      {
          {10, 100, 1000, 10000},
          {10, 100, 10000, 1000},
      },
  };

  for (auto&[arr, ans] : tests) {
    assert(Solution().sortByBits(arr) == ans);
  }
}