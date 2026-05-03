module test_jlrscore_reflect

using Test
using JlrsCore
using .JlrsCore: Reflect

b = Reflect.reflect([String])
@test b isa Reflect.Layouts

end # module test_jlrscore_reflect
