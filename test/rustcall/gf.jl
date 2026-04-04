module test_rustcall_gf

using Test
using RustCall

# from RustCall/docs/src/generics.md
code = """
#[no_mangle]
pub extern "C" fn add<T>(a: T, b: T) -> T {
    a + b
}
"""

RustCall.register_generic_function("add", code, [:T])
@test RustCall.is_generic_function("add")

args = [Int32(10), Int32(20)]
result = RustCall.call_generic_function("add", args...)
@test result == 30

arg_types = typeof.(args)
type_params = RustCall.infer_type_parameters("add", arg_types)
@test type_params == Dict(:T => Int32)

func_info = RustCall.get_monomorphized_function("add", type_params)
func_ptr = func_info.func_ptr
return_type = func_info.return_type
@test RustCall.call_rust_function(func_ptr, return_type, args...) == result

end # module test_rustcall_gf
