#!/bin/bash

# 创建根目录的src/lib.rs
mkdir -p src
if [ ! -f src/lib.rs ]; then
    cat > src/lib.rs.liquid << 'EOF'
    //! {{project_name_pascal}} Root Library
    //! 
    //! This is the root library of the {{project_name_pascal}} project.
    //! It re-exports commonly used items from the workspace crates.
    
    pub use {{project_name_snake}}_api as api;
    pub use {{project_name_snake}}_config as config;
    pub use {{project_name_snake}}_container as container;
    pub use {{project_name_snake}}_database as database;
    pub use {{project_name_snake}}_entities as entities;
    pub use {{project_name_snake}}_entity_base as entity_base;
    pub use {{project_name_snake}}_entity_derive as entity_derive;
    pub use {{project_name_snake}}_libs as libs;
    pub use {{project_name_snake}}_rbac as rbac;
    pub use {{project_name_snake}}_services as services;
    pub use {{project_name_snake}}_storage as storage;
    pub use {{project_name_snake}}_task as task;
    EOF
fi

# 检查二进制crate的main.rs
for bin_crate in api task; do
    if [ -f $bin_crate/src/main.rs ]; then
        echo "$bin_crate/src/main.rs: OK"
    else
        echo "$bin_crate/src/main.rs: Missing"
    fi
done

# 检查库crate的lib.rs
for lib_crate in entity-base entity-derive rbac database entities services storage libs config container; do
    if [ -f $lib_crate/src/lib.rs ]; then
        echo "$lib_crate/src/lib.rs: OK"
    else
        echo "$lib_crate/src/lib.rs: Missing"
    fi
done

# 检查.liquid模板文件
echo -e "\nChecking liquid templates:"
for bin_crate in api task; do
    if [ -f $bin_crate/src/main.rs.liquid ]; then
        echo "$bin_crate/src/main.rs.liquid: OK"
    else
        echo "$bin_crate/src/main.rs.liquid: Missing"
    fi
done

for lib_crate in entity-base entity-derive rbac database entities services storage libs config container; do
    if [ -f $lib_crate/src/lib.rs.liquid ]; then
        echo "$lib_crate/src/lib.rs.liquid: OK"
    else
        echo "$lib_crate/src/lib.rs.liquid: Missing"
    fi
done