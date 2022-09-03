cargo build --release
mkdir Build
cd build
"../target/release/bf_compiler.exe" ../%1
yasm -f win64 out.asm -o out.obj
call "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvarsall.bat" x64
link kernel32.lib user32.lib ucrt.lib shell32.lib gdi32.lib msvcrt.lib /subsystem:console out.obj 
cd ../
