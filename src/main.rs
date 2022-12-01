use std::fmt;
use std::fmt::{Display, format, Formatter};
use colored::Colorize;
use std::mem;
use std::ops::Range;

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

struct UnPrintable(i32);

#[derive(Debug)]
struct DebugPrintable(i32);

pub fn header(text: &str) {
    println!("{}", format!("### {} ###", text).green().bold() );
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\n( {}  {} )\n( {}  {} )", self.0, self.1, self.2, self.3)
    }
}

fn transpose(m: &Matrix) -> Matrix {
    // `let` can be used to bind the members of a tuple to variables
    let Matrix(x1, y1, x2, y2) = m;

    return Matrix(*x1, *x2, *y1, *y2)
}

fn basic_exercices_with_types_and_formats() {
    header("Exercices about basic and simple types and formats");
    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2);
    // TODO ^ Try changing `1i32` to `1u32` to see why the type is important

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);

    let tuple = ("prénom", 16, true, (44.234, -12.847));
    let prenom = tuple.0;
    let age = tuple.1;
    let flag = tuple.2;
    let lat = tuple.3.0;
    let long = tuple.3.1;

    println!("(prenom, age, flag, (lattitude, longitude))");
    println!("({}, {}, {}, ({}, {}))", prenom, age, flag, lat, long);
    println!("Tuple print default --> {:?}", tuple);

    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);

    println!("Pi is roughly {}", format!("{pi:.3}", pi=3.14157f64));

    println!("[DISPLAY] Matrix is {}", Matrix(1.2, 2.2, 3.2, 4.2));
    println!("[DEBUG] Matrix is {:?}", Matrix(1.2, 2.2, 3.2, 4.2));

    let m = Matrix(10.1, 20.1, 30.1, 40.1);
    // Transpose modifie la matrice passée en paramètre !!!
    // Si j'ai bien compris, la structure Matrix doit implémenter
    // la fonction Copy (un peu comme pour fmt::Display)
    println!("Matrix is {}", &m);
    println!("Matrix is {}", transpose(&m));
    println!("Matrix is {}", &m);
}

fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

fn display_array(tab: &[i32]) {
    for i in 0..tab.len() {
        println!("tab[{}] = {}", i, tab[i]);
    }
}

fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn slice_array(array: &[i32], chunk: Range<usize>) -> Vec<i32> {
    if array.len() > chunk.start && array.len() >= chunk.end {
        let mut tmp = array[chunk].to_vec();
        return tmp;
    }

    return array.to_vec();
}

fn exercices_with_array() {
    header("Exercises about arrays");
    let tuple = (0, 1, 2, 3, 4);
    println!("This is a tuple {:?}", tuple);

    const SIZE: usize = 4;
    let tab: [i32; SIZE] = [1; SIZE];
    println!("tab is a {} and tab[0] = {}", type_of(&tab), tab[0]);

    display_array(&tab);

    let mut tab2: [i32; SIZE] = [0; SIZE];
    for i in 0..tab2.len() {
        tab2[i] = (i*i) as i32;
    }

    display_array(&tab2);
    display_array(&tab2[2..4]);

    analyze_slice(&tab2[1..3]);

    let mut arr = [12, 74, 87, 93, 23, 97, 62];
    display_array(&arr);
    println!();
    let range = 2..5;
    let mut v = slice_array(&arr, range);
    display_array(&v);
    println!();
}

#[derive(Debug)]
struct Pair(i32, i32);

#[derive(Debug, Copy, Clone)]
struct Point {
    x: f32,
    y: f32
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Point ({}, {})", self.x, self.y)
    }
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}) -- ({}, {})", self.top_left.x, self.top_left.y, self.bottom_right.x, self.bottom_right.y)
    }
}

impl Display for Pair {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Pair ({}, {})", self.0, self.1)
    }
}

fn square(corner: Point, width: f32) -> Rectangle {
    Rectangle{top_left: corner, bottom_right: Point {x: corner.x + width, y:corner.y + width}}
}

fn exercises_with_structures() {
    header("Exercises with data structures");
    let mut pair: Pair = Pair(32, 32);
    let mut point: Point = Point{ x:2.2, y:9.9 };
    let mut rect = square(Point{x: 12.56, y:23.45}, 50.0);

    println!("Voici une structure de pair: {}", pair);
    println!("Voici une structure de point: {}", point);
    println!("Rectangle {}", rect);
}

fn main() {
    basic_exercices_with_types_and_formats();
    exercices_with_array();
    exercises_with_structures();
}
