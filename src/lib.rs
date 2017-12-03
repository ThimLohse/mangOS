#![feature(lang_items)]
#![feature(unique)]
#![feature(const_fn)]
#![feature(const_unique_new)]
#![no_std]
extern crate rlibc;
extern crate volatile;
extern crate spin;
extern crate multiboot2;

#[macro_use]
mod vga_buffer;

#[no_mangle]
pub extern fn rust_main(multiboot_information_address: usize) {
    // ATTENTION: we have a very small stack and no guard page

    //clear screen
    vga_buffer::clear_screen();

    //print all available memory areas
    let boot_info = unsafe{ multiboot2::load(multiboot_information_address) };
    let memory_map_tag = boot_info.memory_map_tag().expect("Memory map tag required");

    println!("memory areas:");
    for area in memory_map_tag.memory_areas(){
        println!("    start: 0x{:x}, length: 0x{:x}", area.base_addr, area.length);
    }

    //read and print the sections of our kernel ELF file
    let elf_sections_tag = boot_info.elf_sections_tag().expect("Elf-sections tag required");
    println!("kernel sections:");
    for section in elf_sections_tag.sections(){
        println!("    addr: 0x{:x}, size: 0x{:x}, flags: 0x{:x}", section.addr, section.size, section.flags);
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
fn test_printing_functions(){

    //Use to print using the writer implementation in module vga_buffer
    vga_buffer::print_something();


    let hello = b"Hello World!";
    let color_byte = 0x1f; // white foreground, blue background

    let mut hello_colored = [color_byte; 24];
    for (i, char_byte) in hello.into_iter().enumerate() {
    hello_colored[i*2] = *char_byte;
}

	// write `Hello World!` to the center of the VGA text buffer
	let buffer_ptr = (0xb8000 + 1988) as *mut _;
    unsafe { *buffer_ptr = hello_colored };

      // println!("{}", { println!("inner"); "outer" });

   //comment away for testing function
   //test_printing_function();
/*
   importing write traits to use its funcitons
   use core::fmt::Write;
   vga_buffer::WRITER.lock().write_str("Hello again");
   write!(vga_buffer::WRITER.lock(), ", some numbers: {}{}", 42, 1.1337);
*/
    loop{}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle]
pub extern fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    println!("\n\nPANIC in {} at line {}:", file, line);
    println!("   {}", fmt);
    loop{}
}
