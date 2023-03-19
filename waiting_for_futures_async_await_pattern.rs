///this function is just an example, appropriately named
/// the result is still asyncronous, even if it looks like syncronous code, you still have to wait for it in main

async fn example(min_len: usize) -> String {
    let content = async_read_file("foo.txt").await;
    if content.len() < min_len {
        content + &async_read_file("bar.txt").await
    } else {
        content
    }
}