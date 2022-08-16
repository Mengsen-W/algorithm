/*
 * @Date: 2022-08-16
 * @LastEditors: mengsen_wang@163.com
 * @LastEditTime: 2022-08-16
 * @FilePath: /algorithm/1656_OrderedStream/OrderedStream.rs
 */

struct OrderedStream {
    arr: Vec<Option<String>>,
    ptr: usize,
}

impl OrderedStream {
    fn new(n: i32) -> Self {
        OrderedStream {
            arr: vec![None; n as usize],
            ptr: 0,
        }
    }

    fn insert(&mut self, id_key: i32, value: String) -> Vec<String> {
        self.arr[id_key as usize - 1] = Some(value);
        let ret = self.arr[self.ptr..]
            .iter()
            .take_while(|x| x.is_some())
            .map(|x| x.clone().unwrap())
            .collect::<Vec<_>>();
        self.ptr += ret.len();
        ret
    }
}

fn main() {
    let mut o = OrderedStream::new(5);
    assert_eq!(o.insert(3, String::from("ccccc")), Vec::<String>::new());
    assert_eq!(
        o.insert(1, String::from("aaaaa")),
        vec![String::from("aaaaa")]
    );
    assert_eq!(
        o.insert(2, String::from("bbbbb")),
        vec![String::from("bbbbb"), String::from("ccccc")]
    );
    assert_eq!(o.insert(5, String::from("eeeee")), Vec::<String>::new());
    assert_eq!(
        o.insert(4, String::from("ddddd")),
        vec![String::from("ddddd"), String::from("eeeee")]
    );
}
