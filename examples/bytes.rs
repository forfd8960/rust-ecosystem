use bytes::{BufMut, BytesMut};

const BUF_SIZE: usize = 4096;

fn main() {
    let mut buf = BytesMut::with_capacity(BUF_SIZE);
    buf.extend_from_slice(b"Hello, World\n");
    buf.put(&b" heiheihei\n"[..]);
    println!("buf: {:?}", buf);

    buf.put_i32(120);
    println!("buf: {:?}", buf);

    buf.put_f32(99.99);
    println!("buf: {:?}", buf);

    // buf will be empty
    // move content to new_buf
    let new_buf = buf.split();
    println!("new_buf: {:?}", new_buf);
    println!("buf: {:?}", buf);

    let mut immutable_buf = new_buf.freeze();
    println!("immutable buf from new_buf: {:?}", immutable_buf);

    // let pos = immutable_buf.binary_search(&10).unwrap();
    let s = "Hello, World".to_string();
    let split_buf = immutable_buf.split_to(s.len());
    println!("immutable_buf: {:?}", immutable_buf);
    println!("split_buf: {:?}", split_buf);
}
