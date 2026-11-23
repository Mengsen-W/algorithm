#include <vector>
using namespace std;

class ProductOfNumbers {
 private:
  vector<int> nums;
  vector<int> zeroCount;
  vector<int> prevNonOne;
  int n;

 public:
  ProductOfNumbers() {
    nums.clear();
    zeroCount.clear();
    prevNonOne.clear();
    n = 0;
  }

  void add(int num) {
    n++;
    nums.push_back(num);

    prevNonOne.push_back(-1);
    if (n > 1) {
      if (nums[n - 2] != 1 && nums[n - 2] != 0) {
        prevNonOne[n - 1] = n - 2;
      } else {
        prevNonOne[n - 1] = prevNonOne[n - 2];
      }
    }

    zeroCount.push_back(num == 0 ? 1 : 0);
    if (n > 1) {
      zeroCount[n - 1] += zeroCount[n - 2];
    }
  }

  int getProduct(int k) {
    int totalZeros = zeroCount[n - 1];
    if (n - k >= 1) {
      totalZeros -= zeroCount[n - 1 - k];
    }
    if (totalZeros > 0) {
      return 0;
    }

    int ans = 1;
    for (int i = n - 1; i >= n - k;) {
      ans *= nums[i];
      i = prevNonOne[i];
    }
    return ans;
  }
};

int main() {
  ProductOfNumbers productOfNumbers = ProductOfNumbers();
  productOfNumbers.add(3);         // [3]
  productOfNumbers.add(0);         // [3,0]
  productOfNumbers.add(2);         // [3,0,2]
  productOfNumbers.add(5);         // [3,0,2,5]
  productOfNumbers.add(4);         // [3,0,2,5,4]
  productOfNumbers.getProduct(2);  // 返回 20 。最后 2 个数字的乘积是 5 * 4 = 20
  productOfNumbers.getProduct(3);  // 返回 40 。最后 3 个数字的乘积是 2 * 5 * 4 = 40
  productOfNumbers.getProduct(4);  // 返回  0 。最后 4 个数字的乘积是 0 * 2 * 5 * 4 = 0
  productOfNumbers.add(8);         // [3,0,2,5,4,8]
  productOfNumbers.getProduct(2);  // 返回 32 。最后 2 个数字的乘积是 4 * 8 = 32
}