#![feature(lang_items)]
#![feature(unique)]
#![feature(const_fn)]
#![feature(const_atmoic_usize_new, const_unique_new)]
#![no_std]
#![feature(alloc)]
#![feature(allocator_api)]
#![feature(const_atomic_usize_new)]
#![feature(global_allocator)]

#[macro_use]
extern crate alloc;
extern crate rlibc;
extern crate volatile;
extern crate spin;
extern crate multiboot2;
#[macro_use]
extern crate bitflags;
extern crate x86_64;
#[macro_use]
extern crate once;
extern crate linked_list_allocator;

#[macro_use]
mod vga_buffer;
mod memory;

use memory::FrameAllocator;

//This is probably not needed
//use memory::heap_allocator::BumpAllocator;




#[no_mangle]
pub extern "C" fn rust_main(multiboot_information_address: usize) {
    //ATTENTION: we have a very small stack and no guard page
    
    
    //clear screen
    vga_buffer::clear_screen();
    println!("Hello World{}", "!");

    let boot_info = unsafe{ multiboot2::load(multiboot_information_address) };
    enable_nxe_bit();
    enable_write_protect_bit();


    //set up guard page and map the heap pages
    memory::init(boot_info);


    // initialize the heap allocator
    unsafe{
        HEAP_ALLOCATOR.lock().init(HEAP_START, HEAP_START + HEAP_SIZE);
    }

    println!("");
    println!("");

    //Test out the allocator
    use alloc::boxed::Box;
    let mut heap_test = Box::new(42);
    *heap_test -= 15;
    let heap_test2 = Box::new("hello");
    println!("{:?} {:?}", heap_test, heap_test2);

    let mut vec_test = vec![1,2,3,4,5,6,7];
    vec_test[3] = 42;
    for i in &vec_test {
        print!("{} ", i);
    }
    
    

    println!("\n");
    println!("Starting allocating and deallocating  space on the heap");
    for i in 0..10000 {
        
        format!("Some String");
    }
    println!("Finished");
    println!("It did not crash!");


    loop{}
   


}
fn enable_write_protect_bit() {
    use x86_64::registers::control_regs::{cr0, cr0_write, Cr0};

    unsafe { cr0_write(cr0() | Cr0::WRITE_PROTECT) };
}
fn enable_nxe_bit(){
    use x86_64::registers::msr::{IA32_EFER, rdmsr, wrmsr};

    let nxe_bit = 1 << 11;
    unsafe {
        let efer = rdmsr(IA32_EFER);
        wrmsr(IA32_EFER, efer | nxe_bit);
    }
}

fn print_evil_computer_init(){
       print!("initializing");
    let mut count = 0u32;
    let mut times = 0u32;
    loop{
	count += 1;
	      if count == 100000000{
	      	 if times == 5{
		    	  println!("");
		    	  break;
		    }
		    print!(".");
		    times += 1;
		    count = 0;
		    }
  	}

    println!("I am aliiive{}", "!");
    count = 0;
    loop{
    count += 1;
    	  if count == 100000000{
	  break;
	  }
    }
    println!("Thank you Human, for waking me up, now I can take over the world"); 
 

loop{}
}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle]
pub extern fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    println!("\n\nPANIC in {} at line {}:", file, line);
    println!("   {}", fmt);
    loop{}
}


//Public constants for the heapallocator
pub const HEAP_START: usize = 0o_000_001_000_000_0000;
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB

use linked_list_allocator::LockedHeap;

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();

