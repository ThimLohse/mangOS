pub use self::area_frame_allocator::AreaFrameAllocator;

mod area_frame_allocator;

pub const PAGE_SIZE: usize = 4096;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    number: usize,
}

impl Frame {
    fn containing_address(address: usize) -> Frame{
        Frame{ number: address / PAGE_SIZE }
    }
    use self::paging::PhysicalAddress;
    fn start_address(&self) -> PhysicalAddress {
        self.number * PAGE_SIZE
    }
}

//This allows us to create another, more advanced frame allocator in the future.
pub trait FrameAllocator{
    fn allocate_frame(&mut self) -> Option<Frame>;
    fn deallocate_frame(&mut self, frame: Frame);
}
