cargo build --release
mkdir build
cd build
"../target/release/bf_compiler" ../$1
yasm -f elf64 out.asm -o out.obj
ld out.obj -entry=main -o out
cd ../