cd data
python2 karz.py
cd ..

files="./data/*.in"
for filepath in $files; do
  ./target/release/maxflow $filepath
done
