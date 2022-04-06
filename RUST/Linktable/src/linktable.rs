use std::fmt::Display;

/// 链表结点
#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

/// 单链表
#[derive(Debug)]
struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    pub length: usize,
}

//结点定义
impl<T> Node<T> {
    //结点初始化
    fn new(data: T) -> Self {
        Self {
            data: data,
            next: None,
        }
    }
}

//链表定义
impl<T> LinkedList<T> {
    //链表初始化
    fn new() -> Self {
        Self {
            head: None,
            length: 0,
        }
    }
    //插入结点
    fn push(&mut self, data: T) {
        // 从传入数据构建要插入的节点
        let new_node = Box::new(Node::new(data));
        // 当前链表为空时, 插入的节点直接作为头节点
        if self.length == 0 {
            self.head = Some(new_node);
            self.length += 1;
        } else {
            // 当结点的next不为空时，一直往后寻找，直到next为空，插入一个新节点new_node
            let mut cur = self.head.as_mut().unwrap();
            while !cur.next.is_none() {
                cur = cur.next.as_mut().unwrap();
            }
            cur.next = Some(new_node);
            self.length += 1;
        }
    }
    //删除最后一个结点
    fn pop(&mut self) -> Option<Box<Node<T>>> {
        //若结点为空，则返回None
        if self.length == 0 {
            return None;
        //若结点个数为1，则删除该结点
        } else if self.length == 1 {
            self.length -= 1;
            let res = self.head.take();
            return res;
        }
        //找到最后一个结点并删除
        let mut cur = self.head.as_mut().unwrap();
        while !(!cur.next.is_none() && cur.next.as_mut().unwrap().next.is_none()) {
            cur = cur.next.as_mut().unwrap();
        }
        let res = cur.next.take();
        self.length -= 1;
        return res;
    }
    /// 翻转链表
    fn reverse(&mut self) {
        let mut prev = None; // 记录遍历链表时的前一个节点
        while let Some(mut node) = self.head.take() {
            self.head = node.next;
            node.next = prev;
            prev = Some(node);
        }
        self.head = prev;
    }
}

//打印链表
impl<T: Display> Display for LinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.head.is_none() {
            // 如果链表为空, 只打印None
            write!(f, "None\n")?;
        } else {
            // 下面将遍历链表, 因为只是打印, 能获取链表各个节点的数据就行, 所以不需要获取所有权
            let mut next = self.head.as_ref();
            while let Some(node) = next {
                write!(f, "{} -> ", node.data)?;
                next = node.next.as_ref();
            }
            write!(f, "None\n")?;
        }
        Ok(())
    }
}
