let future = async_read_file("something.txt");//imagine this function exists and it's asyncronous
let file_content = loop {
    match future.poll(…) { //the arguments don't matter in this case, so I omitted them
        Poll::Ready(value) => break value,
        Poll::Pending => {}, // do nothing because operation didn't complete yet
    }
}