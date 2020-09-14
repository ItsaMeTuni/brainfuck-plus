use crate::ast::{Node, CommandNode, WriteFormat};
use std::io::{stdin, Read};
use std::time::Duration;

pub fn interpret(ast: Vec<Node>, mem_size: usize)
{
    let mut mem: Vec<u8> = vec![0; mem_size];
    let mut ptr: usize = 0;

    let mut scope_stack: Vec<(&Vec<Node>, usize)> = vec![(&ast, 0)];

    let mut cache: u8 = 0;

    loop
    {
        if scope_stack.last().unwrap().1 >= scope_stack.last().unwrap().0.len()
        {
            scope_stack.pop();
        }

        if scope_stack.last().is_none()
        {
            break;
        }

        let (scope, scope_ptr) = scope_stack.last().unwrap();
        let node = scope.get(*scope_ptr).unwrap();

        //print!("\nmem: {:?} cache: {} ptr: {}", &mem, ptr, cache);

        let mem_cell = mem.get_mut(ptr).unwrap();

        match node
        {
            Node::Command(c) =>
                match c
                {
                    CommandNode::Left(times) =>     prev(&mut ptr, *times),
                    CommandNode::Right(times) =>    next(&mut ptr, *times, mem_size),
                    CommandNode::Add(times) =>      (*mem_cell) = mem_cell.overflowing_add(*times as u8).0,
                    CommandNode::Sub(times) =>      (*mem_cell) = mem_cell.overflowing_sub(*times as u8).0,
                    CommandNode::Write(f) => match f
                    {
                        WriteFormat::Ascii => print!("{}", *mem_cell as char),
                        WriteFormat::Decimal => print!("{}", *mem_cell),
                    },
                    CommandNode::Read => (*mem_cell) = wait_for_input(),
                    CommandNode::CacheUp => cache = (*mem_cell),
                    CommandNode::CacheDown => (*mem_cell) = cache,
                },
            Node::Scope(s) =>
                if *mem.get(ptr).unwrap() != 0
                {
                    scope_stack.push((s, 0));
                    continue;
                },
        }

        let (_, scope_ptr) = scope_stack.last_mut().unwrap();
        *scope_ptr += 1;
    }

    println!("\nmem {:?}; ptr {}; cache {};\n", &mem, ptr, cache);
}

fn prev(ptr: &mut usize, times: usize)
{
    let (new_val, overflow) = ptr.overflowing_sub(times);

    if overflow
    {
        panic!("Memory pointer out of bounds");
    }
    else
    {
        *ptr = new_val;
    }
}

fn next(ptr: &mut usize, times: usize, mem_size: usize)
{
    if *ptr + times >= mem_size
    {
        panic!("Memory pointer out of bounds");
    }
    else
    {
        *ptr += times;
    }
}

pub fn wait_for_input() -> u8
{
    let mut buf = [0 as u8; 1];

    let mut reader = stdin();

    while reader.lock().read(&mut buf).unwrap() == 0
    {
        std::thread::sleep(Duration::from_millis(200));
    }

    buf[0]
}