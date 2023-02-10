pub trait Core {
    fn inc_val(&mut self) -> anyhow::Result<&mut Self>;
    fn dec_val(&mut self) -> anyhow::Result<&mut Self>;
    fn inc_ptr(&mut self) -> anyhow::Result<&mut Self>;
    fn dec_ptr(&mut self) -> anyhow::Result<&mut Self>;
    fn start_loop(&mut self) -> anyhow::Result<&mut Self>;
    fn end_loop(&mut self) -> anyhow::Result<&mut Self>;
}
