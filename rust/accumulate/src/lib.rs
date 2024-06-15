pub fn map<T, U, F>(input: Vec<T>, mut function: F) -> Vec<U>
where
    F: FnMut(T) -> U,
{
    let mut v = Vec::new();
    for i in input {
        v.push(function(i));
    }
    v
}
