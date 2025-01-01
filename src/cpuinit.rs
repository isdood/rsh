// src/cpuinit.rs
use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor};
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    static ref GDT: (GlobalDescriptorTable, [u64; 3]) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let data_selector = gdt.add_entry(Descriptor::kernel_data_segment());
        (gdt, [0, code_selector.0, data_selector.0])
    };
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

pub fn init() {
    GDT.0.load();
    unsafe { 
        x86_64::instructions::tables::load_tss(GDT.1[1].try_into().unwrap());
        x86_64::instructions::tables::load_tss(GDT.1[2].try_into().unwrap());
    }
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}