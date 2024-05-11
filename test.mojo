async def main():
    pass


async fn main[
    a: DType = DType.int32, *, b: Int = 1
](
    inout a: List[Int], owned b: Span[Scalar[a], _, _], borrowed c: Int
) raises -> None:
    var a = 1
    a = 2


struct MyStruct[A: CollectionElement]:
    var a: Int

    fn __init__(inout self):
        pass
