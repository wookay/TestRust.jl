module test_rustcall_cargo

using Test
using RustCall

turboquant_path = normpath(@__DIR__, "../../sources/turboquant-rs")
@test isdir(turboquant_path)
deps = [
    RustCall.DependencySpec("turboquant", path=turboquant_path)
]
@test RustCall.validate_dependencies(deps) === nothing

rust"""
//! ```cargo
//! [dependencies]
//! turboquant = "0.1"
//! ```

use turboquant::*;
"""

end # module test_rustcall_cargo
