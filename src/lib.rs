use std::io::{Read, Write};
use web_sys::window;

use wasm_bindgen::prelude::*;
use bf_rs::{interpreter, token};

#[wasm_bindgen(js_namespace = window)]
extern  "C"{
    fn prompt(message: &str) -> String;
}

#[wasm_bindgen]
pub struct InterpreterContext {
    interpreter: interpreter::Interpreter,
}

struct WebReader{}
struct WebWriter{}

impl Read for WebReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        window().unwrap()
            .prompt()
            .unwrap()
            .unwrap()
            .as_bytes()
            .read(buf)
    }
}

impl Write for WebWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let element = window().unwrap()
            .document().unwrap()
            .get_element_by_id("output")
            .unwrap();

        let inner_conetnt = element.inner_html();
        element.set_inner_html(&(inner_conetnt + &String::from_utf8(buf.to_vec()).unwrap()));
        // element.set_inner_html(&String::from_utf8(buf.to_vec()).unwrap());
            

        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[wasm_bindgen]
impl InterpreterContext {
    pub fn step(
        &mut self,
    ){
        self.interpreter.step(&mut WebReader{}, &mut WebWriter{});
    }

    pub fn run(
        &mut self,
    ){
        self.interpreter.run(&mut WebReader{}, &mut WebWriter{});
    }
}

#[wasm_bindgen]
pub fn new_interpreter(string: &str) -> InterpreterContext {
    let tokens = token::tokenize(string);
    let interpreter = interpreter::Interpreter::new(tokens);
    InterpreterContext { interpreter }
}