/// 思路：
/// 栈stack1用来存放插入的数据
/// 栈stack2用来存放删除的数据
/// 当插入时，直接在stack1后面追加
/// 当要删除时，分情况：
///     1.stack2为空时，我们需要弹出stack1的元素，然后放到stack2，这时stack2的第一个元素就是stack1第一个插入的元素，
///       直接弹出stack2的元素就OK
///     2.stack2不为空时，直接弹出stack2队尾的元素。

struct CQueue {
    stack1: Vec<i32>,
    stack2: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CQueue {
    fn new() -> Self {
        Self {
            stack1: Vec::new(),
            stack2: Vec::new(),
        }
    }

    fn append_tail(&mut self, value: i32) {
        self.stack1.push(value);
    }

    fn delete_head(&mut self) -> i32 {
        // // 先判断stack2是否为空，不为空则直接弹出元素
        // if self.stack2.len() > 0 {
        //     return  self.stack2.pop().unwrap();
        // }
        //
        // // 当stack2为空时，弹出stack1的元素放入stack2中
        // // 且当stack1为空时，返回-1
        // if self.stack1.len() == 0 {
        //     return -1;
        // }
        //
        // while let Some(i) = self.stack1.pop(){
        //     self.stack2.push(i);
        // }
        //
        // self.stack2.pop().unwrap()

        if self.stack2.is_empty() {
            while let Some(i) = self.stack1.pop() {
                self.stack2.push(i);
            }
        }
        self.stack2.pop().unwrap_or(-1)
    }
}

fn main() {}