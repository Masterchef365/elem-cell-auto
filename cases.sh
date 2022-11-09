output=output
width=1000
height=1000
bin=./target/release/elem-cell-auto

mkdir $output

for x in {0..255}; do
    $bin $x $width $height $output/${x}.pbm
    echo $x
done
