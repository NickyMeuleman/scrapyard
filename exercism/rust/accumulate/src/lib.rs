// Because the question says 
// Keep your hands off that collect/map/fmap/whatchamacallit functionality provided by your standard library! 
// Solve this one yourself using other basic tools instead.
pub fn map<Input, Output, F: FnMut(Input) -> Output>(
    input: Vec<Input>,
    mut function: F,
) -> Vec<Output> {
    let mut result = Vec::new();
    for item in input {
        result.push(function(item))
    }
    result
}

// Else, I would write this:
// pub fn map<Input, Output, F: FnMut(Input) -> Output>(
//     input: Vec<Input>,
//     function: F,
// ) -> Vec<Output> {
//     input.into_iter().map(function).collect()
// }