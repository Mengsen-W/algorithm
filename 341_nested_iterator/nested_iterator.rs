/*
 * @Date: 2021-03-23 08:51:09
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-03-23 08:53:20
 */

 #[derive(Debug, PartialEq, Eq)]
 pub enum NestedInteger {
   Int(i32),
   List(Vec<NestedInteger>)
 }

struct NestedIterator(Vec<i32>);

impl NestedIterator {
    fn new(nestedList: Vec<NestedInteger>) -> Self {
        let mut v=collect(nestedList);
        v.reverse();
        Self(v)
    }
    #[inline(always)]
    fn next(&mut self) -> i32 {
        self.0.pop().unwrap()
    }
    #[inline(always)]
    fn has_next(&self) -> bool {
        self.0.len()!=0
    }
}
fn collect(nestedList: Vec<NestedInteger>)->Vec<i32>{
    nestedList.into_iter().map(|x|match x{
        NestedInteger::Int(x)=>vec![x],
        NestedInteger::List(x)=>collect(x)
    }).flatten().collect()
}

/**
 * Your NestedIterator object will be instantiated and called as such:
 * let obj = NestedIterator::new(nestedList);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */
