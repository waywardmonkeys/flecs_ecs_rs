#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_macros)]

pub mod core {
    #![allow(non_snake_case)]
    pub mod Type;
    pub mod archetype;
    pub mod builder;
    pub mod c_binding;
    pub mod c_types;
    pub mod column;
    pub mod component;
    pub mod component_ref;
    pub mod component_registration;
    pub mod entity;
    pub mod entity_view;
    pub mod enum_type;
    pub mod filter;
    pub mod filter_builder;
    pub mod id;
    pub mod iter;
    pub mod iterable;
    pub mod lifecycle_traits;
    pub mod query;
    pub mod query_builder;
    pub mod scoped_world;
    pub mod table;
    pub mod term;
    pub mod world;
    pub mod utility {
        pub mod errors;
        pub mod functions;
        pub mod log;
        pub mod macros;
        pub mod traits;
        pub mod types;
    }
    pub mod data_structures {
        pub mod pair;
    }
}

pub mod addons {}