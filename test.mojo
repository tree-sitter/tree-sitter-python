alias a = 1


struct MyStruct[A: CollectionElement]:
    var a: Int

    fn __init__(inout self):
        self.a = 1
