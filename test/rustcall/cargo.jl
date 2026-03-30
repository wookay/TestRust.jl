module test_rustcall_cargo

using Test
using RustCall

prj_path = normpath(@__DIR__, "../crates/my_rustcall")
@test isdir(prj_path)
deps = [
    RustCall.DependencySpec("my_rustcall", path=prj_path)
]
@test RustCall.validate_dependencies(deps) === nothing

rust"""
#[julia]
fn test_func() {
}
"""

@test test_func() === nothing

end # module test_rustcall_cargo
