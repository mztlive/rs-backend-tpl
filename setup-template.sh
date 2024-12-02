#!/bin/bash

# 为二进制crate创建main.rs
mkdir -p api/src
mkdir -p task/src

# 为库crate创建lib.rs
for crate in entity-base entity-derive rbac database entities services storage libs config container; do
    mkdir -p $crate/src
    if [ ! -f $crate/src/lib.rs.liquid ]; then
        cat > $crate/src/lib.rs.liquid << 'EOF'
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
EOF
    fi
done 