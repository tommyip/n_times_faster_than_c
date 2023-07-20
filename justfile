set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

compare:
  cargo clean
  
  mv .cargo/config.toml .cargo/config.toml.disabled
  cargo bench --no-run
  cargo bench

  mv .cargo/config.toml.disabled .cargo/config.toml
  cargo bench --no-run
  cargo bench > compare.log