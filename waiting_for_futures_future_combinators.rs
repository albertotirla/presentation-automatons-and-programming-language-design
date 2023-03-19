///this code does not compile!
///this type is a future which maps another one which asyncronously returns a string to a function that returns a string's length
/// in this case, both operations become one async compound, the numeric result representing the length is the only thing a caller sees, while the entire machinery runs in an asyncronous way
/// many functions could be composed like this, one after the other, untill the whole call graph is asyncronous
/// then, main is the only syncronous function, which drives the entire thing by a single poling of a top-level future

struct StringLen<F> {
    inner_future: F,
}

impl<F> Future for StringLen<F> where F: Future<Output = String> {
    type Output = usize;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
        match self.inner_future.poll(cx) {
            Poll::Ready(s) => Poll::Ready(s.len()),
            Poll::Pending => Poll::Pending,
        }
    }
}

fn string_len(string: impl Future<Output = String>)
    -> impl Future<Output = usize>
{
    StringLen {
        inner_future: string,
    }
}

// Usage
fn file_len() -> impl Future<Output = usize> {
    let file_content_future = async_read_file("foo.txt");
    string_len(file_content_future)
}