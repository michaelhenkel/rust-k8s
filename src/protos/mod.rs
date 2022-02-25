pub mod k8s {
    pub mod io {
        pub mod apimachinery {
            pub mod pkg {
                pub mod apis {
                    pub mod meta {
                        pub mod v1 {
                            include!("k8s.io.apimachinery.pkg.apis.meta.v1.rs");
                        }
                    }
                }
                pub mod runtime {
                    pub mod schema {
                        include!("k8s.io.apimachinery.pkg.runtime.schema.rs");
                    }
                    include!("k8s.io.apimachinery.pkg.runtime.rs");
                }
            }
        }
    }
}
