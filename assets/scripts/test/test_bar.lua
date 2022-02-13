function on_exit()
  Logger:info("quitting game");
  App.send_message("quit");
end
