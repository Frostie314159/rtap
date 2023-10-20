# RTAP
Radiotap parsing in rust using iterators.
## Notes
### Vendor namespaces
The current implementation doesn't allow for vendor namespaces. As soon as a vendor namespace is encountered the iterator will eagerly skip the subsequent bytes and continues parsing after.
### Iterators
Iterators are small but not empty structures. This library creates an iterator over Radiotap fields from an iterator over bytes. The produced iterator is lazy and only parse the next field when you call next. 
The field iterator itself can get quite large(close to 1.1kB) so avoid copying it around. The byte iterator shouldn't be an owning iterator, since it's copied multiple times internally and this can create quite the overhead. [Proof](https://gist.github.com/rust-play/ac311ad3aa056c33dfdd681a805f5495)
### Panics
This crate was designed to never panic and instead and return either an `Err` or `None`. We use the [no_panic](https://github.com/dtolnay/no-panic) crate to prove, that exposed functions can in no possible way panic.
### Invalid fields
If Rtap fails to parse a field it will return `None`, since we can't possibly guarantee that any subsequent fields were parsed correctly(i.e. it's fused).
### Compiler
A nightly compiler is required, due to the use of unstable libary features. However, since three of the four used features are just iterator adaptors, so you won't get any ICEs.
