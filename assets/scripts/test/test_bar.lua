function initialize(_data)
  print("initialized");
end

function update(document)
  print("updated");
end

function on_exit()
  Logger:info("quitting game");
  App2:request_exit();
end
