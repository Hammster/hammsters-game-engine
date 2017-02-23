use std::io::prelude::*;
use engine;

#[derive(Debug)]
pub enum SmoothingGroup{
    Off,
    Value(String),
} 

#[derive(Debug)]
pub struct Obj {
    o: String,
    mtllib: String,
    g: String,
    usemtl: String,
    v: Vec<Vec<f64>>,
    vn: Vec<Vec<f64>>,
    vt: Vec<Vec<f64>>,
    f: Vec<Vec<Vec<u32>>>,
    s: SmoothingGroup,
}

impl Obj {
    pub fn new () -> Self {
        let cube_buffer = engine::util::file::read_file_buffer("resources/models/Cube/Cube.obj");
        let mut o = String::new();
        let mut mtllib = String::new();
        let mut g = String::new();
        let mut usemtl = String::new();
        let mut v = vec![];
        let mut vn = vec![];
        let mut vt = vec![];
        let mut f = vec![];
        let mut s : SmoothingGroup = SmoothingGroup::Off;

        for (num, line) in cube_buffer.lines().enumerate() {
            let l = line.unwrap();

            match l.split_whitespace().nth(0).unwrap() {
                "o" => o = l.split_whitespace().nth(1).expect("The object name is not valid").to_string(),
                "mtllib" => mtllib = l.split_whitespace().nth(1).expect("The material name is not valid").to_string(),
                "v" => {
                    let mut ln : Vec<f64> = 
                        l.split_whitespace()
                        .map(|x| x.to_string().parse().unwrap_or(0.0) )
                        .collect();

                    ln.remove(0);
                    v.push(ln);
                },
                "vn" => {
                    let mut ln : Vec<f64> = 
                        l.split_whitespace()
                        .map(|x| x.to_string().parse().unwrap_or(0.0) )
                        .collect();

                    ln.remove(0);
                    vn.push(ln);
                },
                "f" => {
                    let mut wrapper : Vec<Vec<u32>> = vec![];
                    let mut ln : Vec<_> = 
                        l.split_whitespace()
                        .map(|x| x.to_string() )
                        .collect();
                        
                    ln.remove(0);

                    for element in &ln { 
                        let mut el : Vec<u32> = 
                            element.split_terminator("/")
                            .map(|x| x.to_string().parse().unwrap_or(0) )
                            .collect();
                        
                        if el.len() == 1 {
                            el.push(0)
                        }

                        if el.len() == 2 {
                            el.push(0)
                        }
                        
                        wrapper.push(el);
                    }
                    
                    f.push(wrapper);
                },
                _ => continue,
            }
        }
        
        Obj {
            o: o,
            mtllib: mtllib,
            g: g,
            usemtl: usemtl,
            v: v,
            vn: vn,
            vt: vt,
            s: s,
            f: f
        }
    }
}