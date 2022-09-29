

trait Excutable{
    type Output;
    fn excute(&self) -> Self::Output;
}