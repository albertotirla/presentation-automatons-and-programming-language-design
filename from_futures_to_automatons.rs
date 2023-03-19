struct StartState {
    min_len: usize,
}

struct WaitingOnFooTxtState {
    min_len: usize,
    foo_txt_future: impl Future<Output = String>,
}

struct WaitingOnBarTxtState {
    content: String,
    bar_txt_future: impl Future<Output = String>,
}

struct EndState {}
enum ExampleStateMachine {
    Start(StartState),
    WaitingOnFooTxt(WaitingOnFooTxtState),
    WaitingOnBarTxt(WaitingOnBarTxtState),
    End(EndState),
}
impl Future for ExampleStateMachine {
    type Output = String; // return type of `example`

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        loop {
            match self {
                ExampleStateMachine::Start(state) => {
                    // taken from the original `example` function, see above
                    let foo_txt_future = async_read_file("foo.txt");
                    // `.await` operation is a state transition, followed by a saving of required state from this one
                    let state = WaitingOnFooTxtState {
                        min_len: state.min_len,
                        foo_txt_future,
                    };
                    *self = ExampleStateMachine::WaitingOnFooTxt(state);
                }
                ExampleStateMachine::WaitingOnFooTxt(state) => {
                    match state.foo_txt_future.poll(cx) {
                        //propagate Pole::Pending in case the future isn't ready, we stop as well, since there's nothing to do, so state remains the same till pole is called again
                        Poll::Pending => return Poll::Pending,
                        Poll::Ready(content) => {
                            // taken from `example` again, after the first .await
                            if content.len() < state.min_len {
                                let bar_txt_future = async_read_file("bar.txt");
                                // `.await` operation, again, state transition
                                let state = WaitingOnBarTxtState {
                                    content,
                                    bar_txt_future,
                                };
                                *self = ExampleStateMachine::WaitingOnBarTxt(state);
                            } else {
                                *self = ExampleStateMachine::End(EndState);
                                return Poll::Ready(content);
                            }
                        }
                    }
                }
                ExampleStateMachine::WaitingOnBarTxt(state) => {
                    match state.bar_txt_future.poll(cx) {
                        Poll::Pending => return Poll::Pending,
                        Poll::Ready(bar_txt) => {
                            *self = ExampleStateMachine::End(EndState);
                            //look at the original `example` function for this part
                            return Poll::Ready(state.content + &bar_txt);
                        }
                    }
                }
                ExampleStateMachine::End(_) => {
                    //we shouldn't reach this state. Once the automaton reached state `T`, trying to start it again is a user error
                    panic!("poll called after Poll::Ready was returned");
                }
            }
        }
    }
}
//since the async function is nothing more but a syncronous one returning a future and because the future has already been generated, here's the final minimal `example` function, to close this listing
fn example(min_len: usize) -> ExampleStateMachine {
    ExampleStateMachine::Start(StartState {
        min_len,
    })
}
