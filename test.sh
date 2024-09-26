cargo b && mkdir lua -p && mv target/debug/libharbinger.so lua/harbinger.so -fn
# nvim
set_rtp=":set rtp+=$PWD"
cmd="
:lua require'harbinger'
"
RUST_BACKTRACE=1 nvim -u NONE --headless +"$set_rtp" +"$cmd" +quit
