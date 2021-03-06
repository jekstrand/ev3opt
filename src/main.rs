/* Copyright © 2020, Jason Ekstrand
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice (including the next
 * paragraph) shall be included in all copies or substantial portions of the
 * Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
 * IN THE SOFTWARE.
 */
use std::env;
use std::process;
use std::path::Path;

mod util;
mod rbf;
mod ir;
mod opt;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 && args.len() != 3 {
        println!("Usage: {} filename", args[0]);
        process::exit(1);
    }

    let in_path = Path::new(&args[1]);

    let mut image = match rbf::read_rbf_file(in_path) {
        Ok(i) => i,
        Err(why) => {
            println!("Failed to parse file: {}", why);
            process::exit(1);
        },
    };

    opt::optimize(&mut image);

    print!("{}", image);

    if args.len() == 3 {
        let out_path = Path::new(&args[2]);
        match rbf::write_rbf_file(out_path, &image) {
            Ok(_) => {},
            Err(why) => {
                println!("Failed to write file: {}", why);
                process::exit(1);
            },
        }
    }
}
