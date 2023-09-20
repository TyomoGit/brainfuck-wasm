use std::{io::{Read, Write}, vec};
use web_sys::window;

use wasm_bindgen::prelude::*;
use brainfuck_interpreter::{interpreter::Interpreter, token};

#[wasm_bindgen(js_namespace = window)]
extern "C"{
    fn prompt(message: &str) -> String;
    fn alert(message: &str);
}

#[wasm_bindgen(js_namespace = console)]
extern "C" {
    fn log(message: &str);
}

#[wasm_bindgen]
pub struct InterpreterContext {
    interpreter: Interpreter,
}

struct WebReader{}

impl Read for WebReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        window().unwrap()
            .prompt().unwrap().unwrap()
            .as_bytes()
            .read(buf)
    }
}
struct WebWriter{
    buffer: Vec<u8>
}

impl WebWriter {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new()
        }
    }
}

impl Write for WebWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if !self.buffer.is_empty() {
            log(&format!("{:?}", self.buffer));
            let mut buffer_clone = self.buffer.clone();
            buffer_clone.append(&mut buf.to_vec());
            if let Ok(s) = String::from_utf8(buffer_clone) {
                append_to_output(&s);
                self.buffer.clear();
                return Ok(buf.len());
            };
        }
    
        let output = &String::from_utf8(buf.to_vec()).unwrap_or_else(|_e| {
            if self.buffer.len() >= 4 {
                self.buffer.clear();
                alert("Error: Invalid UTF-8 sequence");
            }

            self.buffer.append(&mut buf.to_vec());
            String::new()
        });
        append_to_output(output);

        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[wasm_bindgen]
impl InterpreterContext {
    pub fn step(&mut self){
        self.interpreter.step(&mut WebReader{}, &mut WebWriter::new());
    }

    pub fn run(&mut self){
        self.interpreter.run(&mut WebReader{}, &mut WebWriter::new());
    }
}

#[wasm_bindgen]
pub fn new_interpreter(string: &str) -> InterpreterContext {
    let tokens = token::tokenize(string).unwrap_or_else(|e| {
        let writer = &mut WebWriter::new();
        let output = format!("<strong><span style='color: red;'>Error: {}</span></strong>", e);
        writer.write_all(output.as_bytes()).unwrap();
        vec![]
    });
    let interpreter = Interpreter::new(tokens);
    InterpreterContext { interpreter }
}

fn append_to_output(string: &str) {
    let element = window().unwrap()
        .document().unwrap()
        .get_element_by_id("output")
        .unwrap();
    let inner_content = element.inner_html();
    element.set_inner_html(&(inner_content + string));
}