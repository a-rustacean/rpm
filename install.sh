cargo build --release
rm -rf $HOME/.local/bin/rpm
ln -s $(pwd)/target/release/rpm $HOME/.local/bin/rpm
chmod a+rx $HOME/.local/bin/rpm
