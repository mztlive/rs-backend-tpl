#!/bin/bash

# 同步api的代码
cp api/src/main.rs api/src/main.rs.liquid
sed -i 's/use config::/use {{project_name_snake}}_config::/g' api/src/main.rs.liquid
sed -i 's/use container::/use {{project_name_snake}}_container::/g' api/src/main.rs.liquid

# 同步其他代码
for crate in entity-core entity-macros rbac database entities services storage libs config container; do
    if [ -f $crate/src/lib.rs ]; then
        cp $crate/src/lib.rs $crate/src/lib.rs.liquid
        # 在这里添加需要的替换
    fi
done 