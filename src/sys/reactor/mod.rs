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
    ),
    PutChar(
        char,
        // callback
        Box<dyn Fn()>
    ),
    PutBuffer(
        // the buffer is in reverse order
        RefCell<Vec<char>>,
        // callback
        Box<dyn Fn()>
    )
}

struct Handle {
    op: Op
}

pub struct Loop {
    id: u64,
    req: Option<BTreeMap<u64, Handle>>,
    dirty: bool
}

impl Loop {
    pub const fn new() -> Self {
        Loop {
            id: 0,
            req: None,
            dirty: false
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
        self.dirty = true;
    }
    pub fn read_char(&mut self, callback: Box<dyn Fn(char)>) {
        self.id += 1;
        self.req.as_mut().unwrap().insert(self.id, Handle {
            op: Op::ReadChar(callback)
        });
        self.dirty = true;
    }
    pub fn put_char(&mut self, c: char, callback: Box<dyn Fn()>) {
        self.id += 1;
        self.req.as_mut().unwrap().insert(self.id, Handle {
            op: Op::PutChar(c, callback)
        });
        self.dirty = true;
    }
    pub fn put_string(&mut self, s: String, callback: Box<dyn Fn()>) {
        self.id += 1;
        self.req.as_mut().unwrap().insert(self.id, Handle {
            op: Op::PutBuffer(
                RefCell::new(s.chars().rev().collect()),
                callback
            )
        });
        self.dirty = true;
    }
    pub fn is_dirty(&self) -> bool {
        self.dirty || global![mini_uart].character_available()
    }
    pub fn run_inner(&mut self) {
        self.dirty = false;
        let mut dirty = false;
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
                    },
                    Op::PutChar(c, callback) => {
                        if global![mini_uart].try_put_char(*c) {
                            callback();
                            return Some(*id);
                        }
                        dirty = true;
                        None
                    },
                    Op::PutBuffer(buffer, callback) => {
                        let mut b = buffer.borrow_mut();
                        if b.is_empty() {
                            callback();
                            return Some(*id);
                        }
                        let c = b.last().unwrap();
                        if global![mini_uart].try_put_char(*c) {
                            b.pop();
                            if b.is_empty() {    
                                callback();
                                return Some(*id);
                            }
                        }
                        dirty = true;
                        None
                    },
                }
            })
            .collect();
        if dirty {
            self.dirty = true;
        }
        for id in called {
            self.req.as_mut().unwrap().remove(&id);
        }
    }
    pub fn run(&mut self) {
        while {
            self.run_inner();
            self.is_dirty()
        } {}
    }
}
