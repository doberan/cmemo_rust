pub trait Display<T> {
    fn start_line(mut self) -> T;
    fn disp_line(mut self) -> T;
    fn sepalate_line(mut self) -> T;
    fn end_line(mut self) -> T;
    fn disp(mut self);
}