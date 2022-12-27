## inertia-algebra

**inertia-algebra** started as a fork of [alga](https://crates.io/crates/ala) which in turn is a fork of [algebra](https://crates.io/crates/algebra).

**inertia-algebra** provides the algebraic traits used in 
[inertia](https://github.com/wjyoumans/inertia) 
but is designed to be generic enough for other use cases.

<!--
It uses a parent/element pattern where a parent (which can be viewed as a 
set) stores information about its elements and can be used as an element 
constructor. This is necessary for types like finite fields and number fields 
where we need an external object to hold the context, and is a common paradigm
in mathematical software like [SageMath](https://www.sagemath.org/), [Magma](https://www.sagemath.org/), etc.

This construction introduces some unique problems: parents need to know the types of 
their elements and vice versa, and (to be able to do anything nontrivial) we want the
compiler to be able to reason about things like "if `A: Parent` then 
`A::Element::Parent == A`" and "if `A: Group` then `A::Element: LoopElement`" and so 
on. This leads to some messy looking trait bounds. However, these are effectively 
hidden from the user via blanket impls: by implementing basic properties like 
`Identity` and `Associative` the corresponding algebraic structures and all their 
inter-relationships are automatically implemented for you.

TODO:
 * better/more concise version of above explanations
 * comment on blanket impls
 * move comments describing properties to docs in properties.rs
 * give Operation methods for each possible op, with all derived from few but 
   overloadable. Then use macros for deriving std::ops (+ From/Assign ops) 
-->
