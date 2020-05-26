/// Escape text for insertion.
pub fn c_escape<T>(input: T) -> String
    where T: AsRef<str>
{
    let input = input.as_ref();
    let mut chr = Vec::with_capacity(input.len());
    for c in input.chars() {
	match c {
	    '\\' => {
		chr.push('\\');
		chr.push('\\');
	    },
	    '\"' => {
		chr.push('\\');
		chr.push('\"');
	    },
	    _ => chr.push(c),
 	};
    }

    chr.into_iter().collect()
}
