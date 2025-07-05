#include <cassert>
#include <unordered_map>
#include <vector>

using namespace std;

class FindSumPairs {
 private:
  vector<int> nums1, nums2;
  unordered_map<int, int> cnt;

 public:
  FindSumPairs(vector<int>& nums1, vector<int>& nums2) {
    this->nums1 = nums1;
    this->nums2 = nums2;
    for (int num : nums2) {
      ++cnt[num];
    }
  }

  void add(int index, int val) {
    --cnt[nums2[index]];
    nums2[index] += val;
    ++cnt[nums2[index]];
  }

  int count(int tot) {
    int ans = 0;
    for (int num : nums1) {
      int rest = tot - num;
      if (cnt.count(rest)) {
        ans += cnt[rest];
      }
    }
    return ans;
  }
};

int main() {
  vector<int> nums1{1, 1, 2, 2, 2, 3}, nums2{1, 4, 5, 2, 5, 4};
  FindSumPairs findSumPairs = FindSumPairs(nums1, nums2);
  assert(findSumPairs.count(7) == 8);
  findSumPairs.add(3, 2);
  assert(findSumPairs.count(8) == 2);
  assert(findSumPairs.count(4) == 1);
  findSumPairs.add(0, 1);
  findSumPairs.add(1, 1);
  assert(findSumPairs.count(7) == 11);
}