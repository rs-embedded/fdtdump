extern crate clap;
extern crate fdt_rs;
extern crate memmap;
extern crate unsafe_unwrap;
extern crate owning_ref;

use unsafe_unwrap::UnsafeUnwrap;

use clap::{App, Arg};

use memmap::MmapOptions;

use std::fs::File;
use std::slice;
use std::ptr::read_unaligned;
use std::mem::size_of;

use fdt_rs::base::iters::StringPropIter;
use fdt_rs::base::DevTree;
use fdt_rs::index::{DevTreeIndex, DevTreeIndexNode, DevTreeIndexProp};
use fdt_rs::error::Result as DevTreeResult;
use fdt_rs::prelude::*;

unsafe fn allocate_index(buf: &[u8]) -> DevTreeResult<(Vec<u8>, DevTreeIndex)> {
    let devtree = DevTree::new(buf)?;
    let layout = DevTreeIndex::get_layout(&devtree)?;

    let mut vec = vec![0u8; layout.size() + layout.align()];
    let buf = core::slice::from_raw_parts_mut(vec.as_mut_ptr(), vec.len());
    Ok((vec, DevTreeIndex::new(devtree, buf)?))
}

fn are_printable_strings(mut prop_iter: StringPropIter) -> bool {
    loop {
        match prop_iter.next() {
            Ok(Some(s_ref)) => if s_ref.len() == 0 { return false; },
            Ok(None) => return true,
            Err(_) => return false,
        }
    }
}

impl<'i, 'dt> FdtDumper {
    fn push_indent(&mut self) {
        for _ in 0..self.indent {
            self.dump.push_str("    ");
        }
    }

    fn dump_node_name(&mut self, name: &str)  {
        self.push_indent();
        self.dump.push_str(name);
        self.dump.push_str(" {\n");
    }

    fn dump_node(&mut self, node: &DevTreeIndexNode) -> DevTreeResult<()>{
        let mut name = node.name()?;
        if name.len() == 0 {
            name = "/";
        }
        else {
            name = node.name()?;
        }
        self.dump_node_name(name);
        Ok(())
    }

    fn dump_property(&mut self, prop: DevTreeIndexProp) -> DevTreeResult<()> {
        self.push_indent();

        self.dump.push_str(prop.name()?);

        if prop.length() == 0 {
            self.dump.push_str(";\n");
            return Ok(());
        }
        self.dump.push_str(" = ");

        // Unsafe Ok - we're reinterpreting the data as expected.
        unsafe {
            // First try to parse as an array of strings
            if are_printable_strings(prop.iter_str()) {
                let mut iter = prop.iter_str();
                while let Some(s) = iter.next()? {
                    self.dump.push_str("\"");
                    self.dump.push_str(s);
                    self.dump.push_str("\", ");
                }
                let _ = self.dump.pop();
                let _ = self.dump.pop();
            }
            else if prop.propbuf().len() % size_of::<u32>() == 0 {
                self.dump.push('<');
                for val in prop.propbuf().chunks_exact(size_of::<u32>()) {

                    let v = read_unaligned::<u32>(val.as_ptr() as *const u32);
                    let v = u32::from_be(v);
                    self.dump.push_str(format!("{:#010x} ", v).as_str());
                }
                let _ = self.dump.pop(); // Pop off extra space
                self.dump.push('>');
            }
            else {
                self.dump.push('[');
                for val in prop.propbuf() {
                    self.dump.push_str(format!("{:02x} ", val).as_str());
                }
                let _ = self.dump.pop(); // Pop off extra space
                self.dump.push(']');
            }
        }


        self.dump.push_str(";\n");
        Ok(())
    }

    fn dump_level(&mut self, node: &DevTreeIndexNode) -> DevTreeResult<()> {
        self.dump_node(node)?;
        self.indent += 1;
        for prop in node.props() {
            let _ = self.dump_property(prop)?;
        }
        for child in node.children() {
            let _ = self.dump_level(&child)?;
        }
        self.indent -= 1;
        self.push_indent();
        self.dump.push_str("};\n");
        Ok(())
    }

    fn dump_metadata(&mut self, index: &DevTreeIndex) {
        let fdt = index.fdt();
        self.dump.push_str("/dts-v1/;\n");
        self.dump.push_str(format!("// magic:\t\t{:#x}\n", index.fdt().magic()).as_str());
        let s = fdt.totalsize();
        self.dump.push_str(format!("// totalsize:\t\t{:#x} ({})\n", s, s).as_str());
        self.dump.push_str(format!("// off_dt_struct:\t{:#x}\n", fdt.off_dt_struct()).as_str());
        self.dump.push_str(format!("// off_dt_strings:\t{:#x}\n", fdt.off_dt_strings()).as_str());
        self.dump.push_str(format!("// version:\t\t{:#x}\n", fdt.version()).as_str());
        self.dump.push_str(format!("// boot_cpuid_phys:\t{:#x}\n", fdt.boot_cpuid_phys()).as_str());
        self.dump.push_str(format!("// last_comp_version:\t{:}\n", fdt.last_comp_version()).as_str());
        self.dump.push_str(format!("// boot_cpuid_phys:\t{:#x}\n", fdt.boot_cpuid_phys()).as_str());
        self.dump.push_str(format!("// size_dt_strings:\t{:#x}\n", fdt.size_dt_strings()).as_str());
        self.dump.push_str(format!("// size_dt_struct:\t{:#x}\n", fdt.size_dt_struct()).as_str());
        self.dump.push('\n');
    }

    pub(crate) fn dump_tree(buf: &[u8]) -> DevTreeResult<()> {
        let (_vec, index) = unsafe {allocate_index(buf)?};

        let mut dumper = FdtDumper {
            dump: String::new(),
            indent: 0,
        };

        dumper.dump_metadata(&index);
        dumper.dump_level(&index.root())?;
        print!("{}", dumper.dump);
        Ok(())
    }
}

struct FdtDumper {
    dump: String,
    indent: usize,
}

fn main() {
    let args = App::new("fdtdump").version("0.1.0")
        .about("A simple dtb decompiler")
        .arg(Arg::with_name("file").required(true))
        .get_matches();

    // Required - unwrap ok
    unsafe {
        let fname = args.value_of("file").unsafe_unwrap();

        let file = File::open(fname).expect(format!("Unable to open {}", fname).as_str());

        let mmap = MmapOptions::new().map(&file)
            .expect(format!("Unable to map in {}", fname).as_str());

        let slice = slice::from_raw_parts(mmap.as_ptr(), mmap.len());

        FdtDumper::dump_tree(slice).unwrap();
    }
}
