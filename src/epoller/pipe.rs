mod network {
    #[derive(Clone)]
    pub struct PipeWrap {
        pipe_fd: Vec<usize>,
    }
    impl PipeWrap {
        fn new() -> Self {
            let mut fd: Vec<usize> = vec![];
            fd.push(usize::MAX);
            fd.push(usize::MAX);
            Self {
                pipe_fd: fd,
            }
        }
    }
    pub trait PipeWrapTrait {
        fn write(buf: *const (), n: usize) -> usize;
        fn read(buf: *mut (), n: usize) -> usize;
        fn readfd(&self) -> usize;
        fn writefd(&self) -> usize;
        fn clearfd(&self);
    }
    /*
    impl PipeWrapTrait for PipeWrap {
        fn write(buf: *const (), n: usize) -> usize {
            
        }
        fn read(buf: *mut (), n: usize) -> usize {
            
        }
        fn readfd(&self) -> usize {
            return self.pipe_fd[0].clone();
        }
        fn writefd(&self) -> usize {
            return self.pipe_fd[1].clone();
        }
    }
    */
}
