use alloc::prelude::*;
use alloc::collections::BTreeMap;
use core::cell::RefCell;

enum Op {
    ReadLine(
        // buffer
        RefCell<Vec<char>>,
        // callback
        Box<dyn Fn(String)>
    ),
    ReadChar(
        // callback
        Box<dyn Fn(char)>
    )
}

struct Handle {
    op: Op
}

pub struct Loop {
    id: u64,
    req: Option<BTreeMap<u64, Handle>>
}

impl Loop {
    pub const fn new() -> Self {
        Loop {
            id: 0,
            req: None
        }
    }
    pub fn init(&mut self) {
        self.req = Some(BTreeMap::new());
    }
    pub fn read_line(&mut self, callback: Box<dyn Fn(String)>) {
        self.id += 1;
        self.req.as_mut().unwrap().insert(self.id, Handle {
            op: Op::ReadLine(
                RefCell::new(Vec::new()),
                callback
            )
        });
    }
    pub fn read_char(&mut self, callback: Box<dyn Fn(char)>) {
        self.id += 1;
        self.req.as_mut().unwrap().insert(self.id, Handle {
            op: Op::ReadChar(callback)
        });
    }
    pub fn run(&mut self) {
        let character = global![mini_uart].try_get_char();
        let called: Vec<u64> = self.req
            .as_ref()
            .unwrap()
            .iter()
            .filter_map(|(id, handle)| {
                match &handle.op {
                    Op::ReadChar(callback) => {
                        if let Some(mut c) = character {
                            if c == '\r' {
                                c = '\n';
                            }
                            callback(c);
                            return Some(*id);
                        }
                        None
                    },
                    Op::ReadLine(buffer, callback) => {
                        if let Some(mut c) = character {
                            if c == '\r' {
                                c = '\n';
                            }
                            if c == '\n' {
                                callback(buffer.borrow().iter().collect());
                                return Some(*id);
                            } else {
                                buffer.borrow_mut().push(c);
                            }
                        }
                        None
                    }
                }
            })
            .collect();
        for id in called {
            self.req.as_mut().unwrap().remove(&id);
        }
        if global![mini_uart].character_available() {
            self.run();
        }
    }
}
