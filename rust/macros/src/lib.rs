#[macro_export]
macro_rules! hashmap {
    // Empty hashmap case
    () => {
        ::std::collections::HashMap::new()
    };

    // Hashmap with one or more key-value pairs
    (
        $($key:expr => $value:expr),+ // Match one or more key-value pairs
        $(,)?                         // Optionally match a trailing comma
    ) => {{ // First { is macro block, second {  is expression start
        let count = [$( { let _ = &$key; 1 } ),+].len(); // Calculates key count
        let mut hashmap = ::std::collections::HashMap::with_capacity(count); // Create a new preallocated hashmap
        $(
            hashmap.insert($key, $value); // Insert each key-value pair into the hashmap
        )+
        hashmap // Return the populated hashmap
    }}; // First } is expression end, second }; is macro end
}
