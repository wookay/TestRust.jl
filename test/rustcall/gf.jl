module test_rustcall_gf

using Test
using RustCall

code = """
#[no_mangle]
pub extern "C" fn add<T>(a: T, b: T) -> T {
    a + b
}
"""

RustCall.register_generic_function("add", code, [:T])

# Call with different types
result = RustCall.call_generic_function("add", Int32(10), Int32(20))  # => 30
@test result == 30

end # module test_rustcall_gf
