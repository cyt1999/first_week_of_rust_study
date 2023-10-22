pub mod layer_two;
pub fn print_a_to_z() {
    // 因为a ASCII值是97 ，Z 的ASCII值是90，所以需要
    // b'Z'..=b'a' 创建范围 ,rev 反转一个可迭代对象,返回一个新的迭代器，按相反的顺序遍历原始迭代器的元素。
    for c in (b'Z'..=b'a').rev() {
        print!("{} ", char::from(c));
    }
}
